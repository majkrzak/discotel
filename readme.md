# discotel – Discord OpenTelemetry Exporter

> [!WARNING]
> The project is in its early stage. It may retain neither its current shape nor its features. Do not expect any backward compatibility, or, in fact, at this point, any compatibility at all.

Discotel provides OpenTelemetry compatible logging for your Discord server. It observes selected Discord Gateway Events, processes them to a more convenient form, and exports them to an OpenTelemetry collector of your choice.

## Configuration

Configuration is expected to be provided via the environment variables. For convenience, those are also loaded from the `.env` file from the current working directory, although this is mostly for development purposes.

### Discord configuration

`DISCORD_TOKEN` – Discord Bot Token obdaitned from the [Discord Developer Portal](https://discord.com/developers/), from Application settings Bot section.

### Open Telemetry configuration

Discotel utilizes the OpenTelemtry SDK to some extent and therefore relies on subsets of the [OpenTelemetry General SDK Configuration](https://opentelemetry.io/docs/languages/sdk-configuration/general/) and [OpenTelemetry OTLP Exporter Configuration](https://opentelemetry.io/docs/languages/sdk-configuration/otlp-exporter/) environment variables. In particular, the following ones will be useful:

[`OTEL_SERVICE_NAME`](https://opentelemetry.io/docs/languages/sdk-configuration/general/#otel_service_name) – Name of the service. Emitted in `service.name` resource attribute. Required, for example, by Grafana Loki to properly index received logs.
[`OTEL_EXPORTER_OTLP_ENDPOINT`](https://opentelemetry.io/docs/languages/sdk-configuration/otlp-exporter/#otel_exporter_otlp_endpoint) – Base HTTP base endpoint to export logs to.
[`OTEL_EXPORTER_OTLP_LOGS_HEADERS`](https://opentelemetry.io/docs/languages/sdk-configuration/otlp-exporter/#otel_exporter_otlp_headers) – HTTP Headers sent with the logs. This may be used for authenticating the request.
