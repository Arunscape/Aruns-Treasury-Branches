from http.server import BaseHTTPRequestHandler
from mcstatus import MinecraftServer
import json


class handler(BaseHTTPRequestHandler):
    def do_GET(self):

        server = MinecraftServer.lookup("minecraft.woosaree.xyz")

        self.send_response(200)
        self.send_header("Content-type", "application/json")
        self.end_headers()
        response = {
            'status': server.status(),
            'ping': server.ping(),
        }
        self.wfile.write(json.dumps(response))
        return
