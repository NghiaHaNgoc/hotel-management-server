# Hotel management

## [Server link](https://hotel-management-server.fly.dev)
 - Ensure header set `"Content-Type" : "application/json"`

- NOTE:
- `birth_day` field follow format `YYYY-MM-DD` or `YYYY/MM/DD`
- Make sure included Bearer token in header for authorization request

## General end point

#### `/public/amenity/list`
- Method: `GET`
- Get list of all amenity


### `/user/available-room`
- Method: `GET`
- Get available room using `timeFrom`, `timeTo`, `typeRoom`, `adultCapacity`, `kidsCapacity` (all are optional) in query params


#### `/public/type-room/list`
- Method: `GET`
- List all type room

#### `/user/reservation/add`
- Method: `POST`
- Field room_id is optional
- Body:
```json
{
  "room_id": 1,
  "user_id": 4,
  "adult_number": 1,
  "kid_number": 1,
  "checkin_at": "2024-07-20T14:07:04.200294+07:00",
  "checkout_at": "2024-07-21T14:07:04.200294+07:00",
  "type_room_id": 2,
  "total_price": 100000
}
```

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

### Reservation

#### `/admin/reservation/statistic`
- Method: `POST`
- Statistic reservation
- Body:
```json
{
  "x_axis": 1,
  "y_axis": 1,
  "type_room_ids": [
    2,
    3
  ]
}
```

#### `/admin/reservation/summary`
- Method: `GET`
- Summary reservation

#### `/admin/reservation/list`
- Method: `GET`
- List all reservation

#### `/admin/reservation/:reservation_id`
- Method: `GET`
- Get detail reservation

#### `/admin/reservation/:reservation_id/update`
- Method: `POST`
- Update reservation
```json
{
  "checkin_at": "2024-10-24T23:40:00Z",
  "checkout_at": "2024-10-29T00:46:00Z",
  "adult_number": 1,
  "kid_number": 1,
  "total_price": 200000
}
```

#### `/admin/reservation/:reservation_id/open`
- Method: `POST`
- Cancel reservation in status WAITING

#### `/admin/reservation/:reservation_id/cancel`
- Method: `POST`
- Cancel reservation in status WAITING, OPEN, INPROGRESS

### User
#### `/admin/user/add`
- Method: `POST`
- Create user
- Required field : `email`, `position`
- Body:
```json
{
  "email": "test@example.com",
  "position": 2
}
```

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

### Type room

#### `/admin/type-room/add`
- Method: `POST`
- Add new type room
- Field can add: `title`, `view_direction`, `preferential_services`, `size`, `adult_capacity`, `kids_capacity`, `base_price`, `amenities` (amenity's id array), and `images` (link text)
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
- Field can update: `title`, `view_direction`, `preferential_services`, `size`, `adult_capacity`, `kids_capacity`, `base_price`, `amenities` (amenity's id array), `images` (link text)
- Body:
```json
{
  "title": "",
  "amenites": [],
  "images": []
}
```

#### `/admin/type-room/delete/:type_room_id`
- Method: `DELETE`
- Delete type room base on id

#### ~~`/admin/type-room/:type_room_id/image`~~ (deprecated)
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

#### `customer/reservation/list`
- Method: `GET`
- List all reservation of current customer

#### `/customer/reservation/:reservation_id`
- Method: `GET`
- Get detail reservation

#### `/customer/reservation/:reservation_id/update`
- Method: `POST`
- Update reservation
```json
{
  "checkin_at": "2024-10-24T23:40:00Z",
  "checkout_at": "2024-10-29T00:46:00Z",
  "adult_number": 1,
  "kid_number": 1,
  "total_price": 200000
}
```

#### `customer/reservation/:reservation_id/cancel`
- Method: `POST`
- Cancel reservation in status WAITING, OPEN, INPROGRESS
