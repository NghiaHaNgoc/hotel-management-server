# Hotel management

## [Server link](https://hotel-management-server.fly.dev)
 - Ensure header set `"Content-Type" : "application/json"`

- NOTE:
- `birth_day` field follow format `YYYY-MM-DD` or `YYYY/MM/DD`
- Make sure included Bearer token in header for authorization request

## General end point

### `/employee/sign-in` and `/customer/sign-in`
- Method: `POST`
- Body:
 ```json
{
   "email": "your email",
   "password": "your password"
}
```

### `/user/profile`
- Method: `GET`
- get user profile infomation

### `/user/profile`
- Method: `POST`
- Update user profile
- Field can update: `firstname`, `surname`, `city`, `district`, `ward`, `address`, `phone`, `birth_day`, `gender`, `link_avatar`
- To upload avatar, convert image to base64 string and add to json request
- Body:
```json
{
  "firstname": "",
  "city": "",
  "link_avatar": "base64 string"
}
```

### `/user/change-password`
- Method: `POST`
- Body:
```json
{
  "old_password": "",
  "new_password": ""
}
```

## Admin end point

### `/admin/user/list`
- Method: `GET`
- Get list of all user

### `/admin/user/update/:user_id`
- Method: `POST`
- Field can update: `firstname`, `surname`, `city`, `district`, `ward`, `address`, `phone`, `birth_day`, `gender`, `position`, `salary`, and `status`
- Body:
```json
{
  "firstname": "",
  "surname": ""
}
``` 

## Customer end point

### `/customer/sign-up`
- Method: `POST`
- `firstname`, `surname`, `email`, and `password` are required
- Body:
```json
{
  "firstname": "",
  "surname": "",
  "email": "",
  "password": ""
}
```


