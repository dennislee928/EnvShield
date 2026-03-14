package wrapper

import "testing"

func TestResolveBinaryPrefersOverride(t *testing.T) {
	t.Setenv("ENVSHIELD_BINARY", "/tmp/custom-shield")
	path, err := ResolveBinary()
	if err != nil {
		t.Fatalf("expected override to resolve: %v", err)
	}
	if path != "/tmp/custom-shield" {
		t.Fatalf("expected override path, got %s", path)
	}
}
