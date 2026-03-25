#!/usr/bin/env bash
# Setup: create a small Python file for bat to syntax-highlight.
set -euo pipefail

cd "$FIXTURE_WORK_DIR"

cat > server.py << 'PYTHON'
#!/usr/bin/env python3
"""Simple HTTP server with request logging."""

import json
import logging
from http.server import HTTPServer, BaseHTTPRequestHandler
from datetime import datetime

LOG_FORMAT = "%(asctime)s [%(levelname)s] %(message)s"
logging.basicConfig(level=logging.INFO, format=LOG_FORMAT)
logger = logging.getLogger(__name__)

class RequestHandler(BaseHTTPRequestHandler):
    """Handle incoming HTTP requests."""

    def do_GET(self):
        logger.info(f"GET {self.path} from {self.client_address[0]}")
        if self.path == "/health":
            self._send_json({"status": "ok", "uptime": 42})
        elif self.path == "/api/users":
            users = [
                {"id": 1, "name": "Alice", "role": "admin"},
                {"id": 2, "name": "Bob", "role": "user"},
            ]
            self._send_json(users)
        else:
            self._send_error(404, "Not found")

    def _send_json(self, data, status=200):
        body = json.dumps(data, indent=2).encode()
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        self.wfile.write(body)

    def _send_error(self, code, message):
        self._send_json({"error": message}, status=code)

if __name__ == "__main__":
    server = HTTPServer(("0.0.0.0", 8080), RequestHandler)
    logger.info("Listening on port 8080")
    server.serve_forever()
PYTHON
