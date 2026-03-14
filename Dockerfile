FROM node:20-alpine AS console-builder
WORKDIR /workspace

COPY package.json package-lock.json tsconfig.base.json ./
COPY contracts ./contracts
COPY apps/console/package.json ./apps/console/package.json
COPY packages/api-client/package.json ./packages/api-client/package.json
COPY apps/console ./apps/console
COPY packages/api-client ./packages/api-client

RUN npm ci
RUN npm run generate:types
RUN npm run build --workspace @envshield/console

FROM golang:1.22-alpine AS go-builder
WORKDIR /workspace

COPY services/control-plane/go.mod services/control-plane/go.sum ./services/control-plane/
RUN cd services/control-plane && go mod download

COPY services/control-plane ./services/control-plane
RUN cd services/control-plane && CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -o /out/envshield-control-plane ./cmd/server

FROM alpine:3.20
WORKDIR /app

RUN addgroup -S envshield && adduser -S envshield -G envshield

COPY --from=go-builder /out/envshield-control-plane /app/envshield-control-plane
COPY --from=console-builder /workspace/apps/console/dist /app/console

ENV PORT=8080
ENV ENVSHIELD_STATIC_DIR=/app/console

EXPOSE 8080

USER envshield

CMD ["/app/envshield-control-plane"]

