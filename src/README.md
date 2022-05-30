# Near Chat Application

Example chat front-end application. Uses NEAR [contract](../contract/) as a back-end.
So, it provides an ability for near account to send message to selected chat contact and getting messages for any user.

## Deployment

To start server locally:

```shell
yarn dev
```

To build app for production:
```shell
yarn build
```

## Possible NEAR platform improvements

1. No out-of-the box event notifications through WebSockets.
   With such a feature added this app could dynamically update UI with new messages.
