package main

import (
	"log"
	"net/http"
	"os"

	"github.com/envshield/envshield/services/control-plane/internal/api"
	"github.com/envshield/envshield/services/control-plane/internal/core"
	"github.com/envshield/envshield/services/control-plane/internal/store"
)

func main() {
	publicURL := os.Getenv("ENVSHIELD_PUBLIC_URL")
	if publicURL == "" {
		publicURL = "http://localhost:5173"
	}
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	service := core.NewService(store.NewMemoryStore(), publicURL)
	router := api.NewRouter(service)

	log.Printf("EnvShield control plane listening on :%s", port)
	if err := http.ListenAndServe(":"+port, router.Handler()); err != nil {
		log.Fatal(err)
	}
}
