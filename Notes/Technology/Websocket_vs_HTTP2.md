# Websocket vs. HTTP/2

https://www.infoq.com/articles/websocket-and-http2-coexist/

|                | HTTP/2                      | WebSocket         |
| -------------- | --------------------------- | ----------------- |
| Header         | Compressed (HPACK)          | None              |
| Binary         | YES                         | Binary or Textual |
| Multiplexing   | YES                         | YES               |
| Prioritization | YES                         | NO                |
| Compression    | YES                         | YES               |
| Direction      | Client/Server + Server Push | Bidirectional     |
| Full-Duplex    | YES                         | YES               |

## is HTTP/2 a replacement for push technologies such as WebSocket or SSE? NO

Server Push vs Websocket Bidirectional communication
* Server push only push data down to the client cache
* i.e. the client application does not get notification for the event
