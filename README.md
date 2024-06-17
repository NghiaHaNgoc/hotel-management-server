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

### User
#### `/admin/user/list`
- Method: `GET`
- Get list of all user

#### `/admin/user/update/:user_id`
- Method: `POST`
- Field can update: `firstname`, `surname`, `city`, `district`, `ward`, `address`, `phone`, `birth_day`, `gender`, `position`, `salary`, and `status`
- Body:
```json
{
  "firstname": "",
  "surname": ""
}
```
### Amenity
#### `/admin/amenity/list`
- Method: `GET`
- Get list of all amenity

#### `/admin/amenity/add`
- Method: `POST`
- Body:
```json
{
  "name": "",
  "amenity_type": 1
}
```
#### `/admin/amenity/delete/:amenity_id`
- Method: `DELETE`
- Delete the amenity has this id

### Room type
#### `/admin/type-room/list`
- Method: `GET`
- List all type room

#### `/admin/type-room/add`
- Method: `POST`
- Add new type room
- Field can add: `title`, `view_direction`, `preferential_services`, `size`, `adult_capacity`, `kids_capacity`, `base_price`, `amenities` (amenity's id array), and `images` (base64 array)
- Body:
```json
{
  "title": "",
  "amenites": [],
  "images": []
}
```

#### `/admin/type-room/update/:type_room_id`
- Method: `POST`
- Update type room
- Field can update: `title`, `view_direction`, `preferential_services`, `size`, `adult_capacity`, `kids_capacity`, `base_price`, `amenities` (amenity's id array), `add_images` (base64 array), `delete_images` (id image array)
- Body:
```json
{
  "title": "",
  "amenites": [],
  "add_images": [],
  "delete_images": []
}
```

#### `/admin/type-room/delete/:type_room_id`
- Method: `DELETE`
- Delete type room base on id

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


