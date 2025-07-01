import socket

client =socket.socket(socket.AF_INET6,socket.SOCK_STREAM)
client.connect(('localhost',3001))

recv = client.recv(1024)

msg = """
GET / HTTP/1.1
Host: localhost:3001
User-Agent: curl/8.12.1
Accept: */*

"""

client.send(msg.encode('utf-8'))


print(recv)