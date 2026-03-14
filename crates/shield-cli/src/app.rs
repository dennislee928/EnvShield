use std::{
    collections::BTreeMap,
    thread,
    time::{Duration, Instant},
};

use anyhow::{anyhow, bail, Context, Result};
use clap::{Args, Parser, Subcommand};
use rpassword::prompt_password;

use crate::{
    api::{
        ApiClient, CreateSnapshotRequest, CreateWorkspaceRequest, DeviceResponse,
        EnvironmentStatusResponse, RegisterDeviceRequest,
    },
    crypto::{decrypt_snapshot, encrypt_snapshot, generate_device_identity, verify_manifest},
    runner::{run_command, to_decrypted},
    state::{
        decrypt_local_cache, encrypt_local_cache, AppState, KnownDevice, StateStore, StoredDevice,
    },
};

#[derive(Debug, Parser)]
#[command(name = "shield", version, about = "Zero-knowledge CLI secret injector")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Login(LoginArgs),
    Init(InitArgs),
    Secret {
        #[command(subcommand)]
        command: SecretCommands,
    },
    Push(EnvironmentArgs),
    Pull(EnvironmentArgs),
    Status(EnvironmentArgs),
    Run(RunArgs),
}

#[derive(Debug, Args)]
struct LoginArgs {
    #[arg(long)]
    device_name: Option<String>,
    #[arg(long)]
    approve_as: Option<String>,
}

#[derive(Debug, Args)]
struct InitArgs {
    #[arg(long)]
    workspace: String,
}

#[derive(Debug, Subcommand)]
enum SecretCommands {
    Set(SecretSetArgs),
    List(EnvironmentArgs),
}

#[derive(Debug, Args)]
struct EnvironmentArgs {
    #[arg(long)]
    env: String,
    #[arg(long)]
    workspace: Option<String>,
}

#[derive(Debug, Args)]
struct SecretSetArgs {
    key: String,
    #[arg(long)]
    env: String,
    #[arg(long)]
    workspace: Option<String>,
}

#[derive(Debug, Args)]
struct RunArgs {
    #[arg(long)]
    env: String,
    #[arg(long)]
    workspace: Option<String>,
    #[arg(trailing_var_arg = true, required = true)]
    command: Vec<String>,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let store = StateStore::new()?;
    let mut state = store.load()?;

    match cli.command {
        Commands::Login(args) => login(args, &mut state, &store),
        Commands::Init(args) => init(args, &mut state, &store),
        Commands::Secret {
            command: SecretCommands::Set(args),
        } => secret_set(args, &mut state, &store),
        Commands::Secret {
            command: SecretCommands::List(args),
        } => secret_list(args, &mut state),
        Commands::Push(args) => push(args, &mut state, &store),
        Commands::Pull(args) => pull(args, &mut state, &store),
        Commands::Status(args) => status(args, &state),
        Commands::Run(args) => run_with_env(args, &mut state, &store),
    }
}

fn login(args: LoginArgs, state: &mut AppState, store: &StateStore) -> Result<()> {
    let device_name = args.device_name.unwrap_or_else(|| whoami::devicename());
    let api = ApiClient::new(state.api_base_url.clone(), None);
    let response = api.start_github_auth(&device_name)?;
    maybe_open_browser(&response.verification_url);

    println!("Open {}", response.verification_url);
    println!("User code: {}", response.user_code);

    if let Some(actor_email) = args.approve_as.as_deref() {
        api.approve_device_auth(&response.device_code, actor_email)?;
    }

    let started = Instant::now();
    let token = loop {
        match api.exchange_device_auth(&response.device_code) {
            Ok(token) => break token,
            Err(error) if error.to_string().contains("409") => {
                if started.elapsed() > Duration::from_secs(response.expires_in) {
                    bail!("device auth flow expired");
                }
                thread::sleep(Duration::from_secs(2));
            }
            Err(error) => return Err(error),
        }
    };

    if state.device.is_none() {
        let identity = generate_device_identity();
        state.device = Some(StoredDevice {
            id: None,
            name: device_name,
            age_identity: identity.age_identity,
            age_recipient: identity.age_recipient,
            signing_secret_key: identity.signing_secret_key,
            signing_public_key: identity.signing_public_key,
        });
    }

    state.actor_email = Some(token.actor_email.clone());
    state.token = Some(token.token);
    store.save(state)?;
    println!("Logged in as {}", token.actor_email);
    Ok(())
}

fn init(args: InitArgs, state: &mut AppState, store: &StateStore) -> Result<()> {
    let api = authorized_api(state)?;
    let device = ensure_registered_device(state, &api)?;
    let workspace = api.create_workspace(&CreateWorkspaceRequest {
        name: args.workspace,
        device_id: device.id.clone(),
    })?;
    state.upsert_workspace(&workspace.id, &workspace.name);
    store.save(state)?;
    println!("Workspace {} initialized", workspace.name);
    Ok(())
}

fn secret_set(args: SecretSetArgs, state: &mut AppState, store: &StateStore) -> Result<()> {
    let workspace_id = state.resolve_workspace_id(args.workspace.as_deref())?;
    let mut secrets = load_environment_values(state, &workspace_id, &args.env)?;
    let value = prompt_password(format!("Value for {}: ", args.key))
        .context("failed to read secret from prompt")?;
    secrets.insert(args.key.clone(), value);
    let cache_key = state.local_cache_key()?;
    let encrypted = encrypt_local_cache(&cache_key, &secrets)?;
    let environment_state = state.environment_state_mut(&workspace_id, &args.env)?;
    environment_state.cache = Some(encrypted);
    environment_state.dirty = true;
    store.save(state)?;
    println!("Stored secret {}", args.key);
    Ok(())
}

fn secret_list(args: EnvironmentArgs, state: &mut AppState) -> Result<()> {
    let workspace_id = state.resolve_workspace_id(args.workspace.as_deref())?;
    let secrets = load_environment_values(state, &workspace_id, &args.env)?;
    for key in secrets.keys() {
        println!("{key}");
    }
    Ok(())
}

fn push(args: EnvironmentArgs, state: &mut AppState, store: &StateStore) -> Result<()> {
    let workspace_id = state.resolve_workspace_id(args.workspace.as_deref())?;
    let api = authorized_api(state)?;
    let device = ensure_registered_device(state, &api)?;
    let values = load_environment_values(state, &workspace_id, &args.env)?;
    if values.is_empty() {
        bail!("no secrets found for environment {}", args.env);
    }

    let stored_device = state.device()?.clone();
    let (key_envelopes, secrets, manifest_signature) = encrypt_snapshot(
        &stored_device,
        &device.id,
        &workspace_id,
        &args.env,
        &values,
    )?;

    let snapshot = api.create_snapshot(&CreateSnapshotRequest {
        workspace_id: workspace_id.clone(),
        environment: args.env.clone(),
        created_by_device: device.id.clone(),
        manifest_signature,
        key_envelopes,
        secrets,
    })?;

    let known_device = KnownDevice {
        id: device.id.clone(),
        name: device.name,
        encryption_public_key: device.encryption_public_key,
        signing_public_key: device.signing_public_key,
    };
    state
        .known_devices
        .insert(known_device.id.clone(), known_device);
    let environment_state = state.environment_state_mut(&workspace_id, &args.env)?;
    environment_state.version = snapshot.version;
    environment_state.last_snapshot = Some(snapshot.clone());
    environment_state.dirty = false;
    store.save(state)?;
    println!("Pushed snapshot v{} for {}", snapshot.version, args.env);
    Ok(())
}

fn pull(args: EnvironmentArgs, state: &mut AppState, store: &StateStore) -> Result<()> {
    let workspace_id = state.resolve_workspace_id(args.workspace.as_deref())?;
    let api = authorized_api(state)?;
    let snapshot = api.get_latest_snapshot(&workspace_id, &args.env)?;
    let device = state.device()?.clone();
    let decrypted = decrypt_snapshot(&device, &state.known_devices, &snapshot)?;
    let cache_key = state.local_cache_key()?;
    let encrypted = encrypt_local_cache(&cache_key, &decrypted.secrets)?;

    let environment_state = state.environment_state_mut(&workspace_id, &args.env)?;
    environment_state.version = snapshot.version;
    environment_state.last_snapshot = Some(snapshot);
    environment_state.cache = Some(encrypted);
    environment_state.dirty = false;
    store.save(state)?;
    println!("Pulled latest snapshot for {}", args.env);
    Ok(())
}

fn status(args: EnvironmentArgs, state: &AppState) -> Result<()> {
    let workspace_id = state.resolve_workspace_id(args.workspace.as_deref())?;
    let local_version = state
        .workspaces
        .get(&workspace_id)
        .and_then(|workspace| workspace.environments.get(&args.env))
        .map(|environment| environment.version)
        .unwrap_or(0);
    let api = authorized_api_ref(state)?;
    let response = api.get_environment_status(&workspace_id, &args.env, local_version)?;
    print_status(&response);
    Ok(())
}

fn run_with_env(args: RunArgs, state: &mut AppState, store: &StateStore) -> Result<()> {
    let workspace_id = state.resolve_workspace_id(args.workspace.as_deref())?;
    maybe_warn_outdated(state, &workspace_id, &args.env)?;
    let values = load_environment_values(state, &workspace_id, &args.env)?;

    if let Some(snapshot) = state
        .workspaces
        .get(&workspace_id)
        .and_then(|workspace| workspace.environments.get(&args.env))
        .and_then(|environment| environment.last_snapshot.as_ref())
    {
        let known_device = state
            .known_devices
            .get(&snapshot.created_by_device)
            .ok_or_else(|| anyhow!("snapshot signer is unknown in local state"))?;
        verify_manifest(&known_device.signing_public_key, snapshot)?;
    }

    let status = run_command(&args.command, to_decrypted(values))?;
    store.save(state)?;
    std::process::exit(status);
}

fn load_environment_values(
    state: &mut AppState,
    workspace_id: &str,
    environment: &str,
) -> Result<BTreeMap<String, String>> {
    let cache = state
        .workspaces
        .get(workspace_id)
        .and_then(|workspace| workspace.environments.get(environment))
        .and_then(|environment| environment.cache.clone());
    if let Some(cache) = cache {
        let cache_key = state.local_cache_key()?;
        return decrypt_local_cache(&cache_key, &cache);
    }
    Ok(BTreeMap::new())
}

fn ensure_registered_device(state: &mut AppState, api: &ApiClient) -> Result<DeviceResponse> {
    let device = state.device()?.clone();
    if let Some(device_id) = &device.id {
        let known_device = state
            .known_devices
            .get(device_id)
            .ok_or_else(|| anyhow!("local device registry is missing {}", device_id))?;
        return Ok(DeviceResponse {
            id: known_device.id.clone(),
            name: known_device.name.clone(),
            encryption_public_key: known_device.encryption_public_key.clone(),
            signing_public_key: known_device.signing_public_key.clone(),
            created_at: String::new(),
        });
    }

    let response = api.register_device(&RegisterDeviceRequest {
        name: device.name.clone(),
        encryption_public_key: device.age_recipient.clone(),
        signing_public_key: device.signing_public_key.clone(),
    })?;
    state.device_mut()?.id = Some(response.id.clone());
    state.known_devices.insert(
        response.id.clone(),
        KnownDevice {
            id: response.id.clone(),
            name: response.name.clone(),
            encryption_public_key: response.encryption_public_key.clone(),
            signing_public_key: response.signing_public_key.clone(),
        },
    );
    Ok(response)
}

fn authorized_api(state: &AppState) -> Result<ApiClient> {
    let token = state
        .token
        .clone()
        .ok_or_else(|| anyhow!("not logged in; run `shield login` first"))?;
    Ok(ApiClient::new(state.api_base_url.clone(), Some(token)))
}

fn authorized_api_ref(state: &AppState) -> Result<ApiClient> {
    authorized_api(state)
}

fn maybe_warn_outdated(state: &AppState, workspace_id: &str, environment: &str) -> Result<()> {
    let local_version = state
        .workspaces
        .get(workspace_id)
        .and_then(|workspace| workspace.environments.get(environment))
        .map(|value| value.version)
        .unwrap_or(0);
    let api = authorized_api_ref(state)?;
    let response = api.get_environment_status(workspace_id, environment, local_version)?;
    if response.out_of_date {
        eprintln!(
            "warning: local snapshot is outdated (local v{}, latest v{}). Run `shield pull --env {}` to sync.",
            response.local_version, response.latest_version, environment
        );
    }
    Ok(())
}

fn print_status(response: &EnvironmentStatusResponse) {
    if response.out_of_date {
        println!(
            "Environment {} is behind: local v{}, latest v{}",
            response.environment, response.local_version, response.latest_version
        );
    } else {
        println!(
            "Environment {} is up to date at v{}",
            response.environment, response.latest_version
        );
    }
}

fn maybe_open_browser(url: &str) {
    let command = if cfg!(target_os = "macos") {
        Some(("open", vec![url.to_string()]))
    } else if cfg!(target_os = "linux") {
        Some(("xdg-open", vec![url.to_string()]))
    } else {
        None
    };
    if let Some((binary, args)) = command {
        let _ = std::process::Command::new(binary).args(args).spawn();
    }
}
