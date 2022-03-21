import eventdata_pb2 as ed # Generated from eventdata.proto
import socket
import struct

def get_length(data):
    """
    Returns the length of the data.
    """
    return struct.unpack("<I", data)[0]

def connect_tcp_socket(host, port):
    """
    Connect to a TCP socket
    """
    socket.setdefaulttimeout(15)
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect((host, port))
    return sock

def read_and_print_socket_data(sock):
    """
    Read and print data from the socket
    """

    while True:
        dlength = get_length(sock.recv(4))
        data = b""
        while len(data) < dlength:
            data += sock.recv(dlength - len(data))
        event = ed.Event.FromString(data)
        cm = event.combat_message
        if event.HasField("combat_message") :
            print(cm)
            print(cm.src_actor.name, cm.skillname, cm.dst_actor.name, cm.combat_event.damage)

def main():
    """
    Main function
    """
    sock = connect_tcp_socket('localhost', 12112)
    read_and_print_socket_data(sock)

if __name__ == "__main__":
    main()