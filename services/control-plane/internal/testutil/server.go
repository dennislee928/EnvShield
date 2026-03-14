package testutil

import (
	"net/http/httptest"

	"github.com/envshield/envshield/services/control-plane/internal/api"
	"github.com/envshield/envshield/services/control-plane/internal/core"
	"github.com/envshield/envshield/services/control-plane/internal/store"
)

func NewTestServer() *httptest.Server {
	memory := store.NewMemoryStore()
	service := core.NewService(memory, "http://localhost:5173")
	router := api.NewRouter(service, "")
	return httptest.NewServer(router.Handler())
}
