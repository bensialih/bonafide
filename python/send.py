import json
import socket
import time

port: int = 3000
host: str = 'localhost'

with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as sock:
    sock.connect((host, port))

    message: str = json.dumps(
        dict(
            ip = '127.0.0.1',
            datetime = '2019-08-15T17:41:18.106108'
        )
    )

    sock.send(message.encode())


