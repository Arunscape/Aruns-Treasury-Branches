from http.server import BaseHTTPRequestHandler
from mcstatus import MinecraftServer
import json
import urllib.request

def get_stats():
    server = MinecraftServer.lookup("minecraft.woosaree.xyz")
    status = server.status()

    external_ip = urllib.request.urlopen('https://ifconfig.me').read().decode('utf8')
    
    players = []
    if status.players.sample is not None:
        players = list(map(lambda x: {'name': x.name, 'id': x.id}, status.players.sample))

    return {
        "ping": {
            "from": external_ip,
            "time": status.latency,
        },
        # "players": status.players,
        "max_players": status.players.max,
        "players_sample": players,
        "favicon": status.favicon,
        "version": status.version.name,
        "protocol": status.version.protocol,
        "description": status.description,
        "num_online": status.players.online,
        # "query": server.query(),
    }


class handler(BaseHTTPRequestHandler):
    def do_GET(self):

        self.send_response(200)
        self.send_header("Content-type", "application/json")
        self.end_headers()
        response = get_stats()
        self.wfile.write(json.dumps(response).encode())
        return

if __name__ == "__main__":
    import pprint
    pp = pprint.PrettyPrinter(indent=4)
    pp.pprint(get_stats())