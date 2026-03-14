package wrapper

import (
	"errors"
	"os"
	"path/filepath"
)

func ResolveBinary() (string, error) {
	if override := os.Getenv("ENVSHIELD_BINARY"); override != "" {
		return override, nil
	}

	candidates := []string{
		filepath.Join("..", "..", "target", "release", "shield"),
		filepath.Join("..", "..", "target", "debug", "shield"),
	}
	for _, candidate := range candidates {
		if _, err := os.Stat(candidate); err == nil {
			return candidate, nil
		}
	}
	return "", errors.New("unable to locate EnvShield Rust binary; build shield-cli or set ENVSHIELD_BINARY")
}
