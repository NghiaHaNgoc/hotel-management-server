# Hotel management

## [Server link](https://hotel-management-server.fly.dev)
 - Ensure header set `"Content-Type" : "application/json"`

- NOTE:
- `birth_day` field follow format `YYYY-MM-DD` or `YYYY/MM/DD`
- Make sure included Bearer token in header for authorization request

## General end point

### `/user/available-room`
- Method: `GET`
- Get available room using `timeFrom`, `timeTo`, `typeRoom` (optional), `adultCapacity` (optional), `kidsCapacity` (optional) in query params


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
  "type": 1
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

#### `/admin/type-room/:type_room_id/image`
- Method: `POST`
- Upload image of type room
- Body:
```json
{
  "link": "base64"
}
```

### Room
#### `/admin/room/list`
- Method: `GET`
- List all type room

#### `/admin/room/add`
- Method: `POST`
- Add new room
- Field can add: `type_room_id`, `room_number`, `floor`, and `status`
- Body:
```json
{
  "type_room_id": 1,
  "room_number": "A1",
  "floor": 1
}
```

#### `/admin/room/update/:room_id`
- Method: `POST`
- Update room
- Field can update: `type_room_id`, `room_number`, `floor`, and `status`
- Body:
```json
{
  "type_room_id": 1,
  "room_number": "A1",
  "floor": 1
}
```

#### `/admin/room/delete/:room_id`
- Method: `DELETE`
- Delete room base on id


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

### Room
#### `/customer/room/list`
- Method: `GET`
- List all type room

#### `/customer/room/detail/:room_id`
- Method: `GET`
- List detail of a room

### Reservation
#### `/customer/reservation/add`
- Method: `POST`
- Body:
```json
{
  "room_id": 5,
  "total_price": 100000,
  "checkin_at": "2024-06-28T06:15:12.830Z",
  "checkout_at": "2024-06-28T12:15:12.830Z"
}
```

#### `customer/reservation/list`
- Method: `GET`
- List all reservation of current customer

#### `customer/reservation/:reservation_id/cancel`
- Method: `POST`
- Cancel reservation
