# Cost Insurance and Freight Api Specification

## Get CIF
Endpoint: **GET** `/api/v1/user/data`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "Get CIF Success",
    "data": {
        "autonid": 1,
        "fullname": "New York",
        "register_date": "London",
        "mobile_phone": "2022-01-01",
        .........
        ........
    }
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Get CIF Failed",
    "data": "User not found"
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Get CIF Failed",
    "data": "Internal Server Error"
}
```

## Update CIF State 1
Endpoint: **POST** `/api/v1/user/data-pribadi`

Request Header:
- Authorized token (Cookies)

Request Body:
```json
{
    "email": "example@gmail.com",
    "mobile_phone": "6282323443535",
    "full_name": "Dani",
    "mother_name": "Dina",
    "idcard_number": "123456789101112",
    "nationality": 6,
    "sex": 2,
    "birth_place": "PEKALONGAN",
    "birth_date": "2024-03-03",
    "birth_country": "sd",
    "religion": 2,
    "marital_status": 1,
    "education": 3,
    "education_text": "",
    "copy_id": false,
    "idcard_expireddate": "2024-03-03",
    "idcard_city": 6,
    "idcard_district": "as",
    "idcard_subdistrict": "sd",
    "idcard_rt": "34",
    "idcard_rw": "23",
    "idcard_address": "43",
    "idcard_zipcode": "43",
    "domicile_city": 9,
    "domicile_district": "hh",
    "domicile_subdistrict": "hh",
    "domicile_rt": "88",
    "domicile_rw": "999",
    "domicile_address": "hh",
    "domicile_zipcode": "9",
    "idcard_file": "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/.......",
    "selfie_file": "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/.......",
    "signature_file": "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/......."
}
```

Response Body(200):
```json
{
    "result": true,
    "message": "Save CIF Stage 1 Success"
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Invalid request",
    "data": "ID Card is required, .etc"
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "Please check connection server"
}
```

## Update CIF State 2
Endpoint: **POST** `/api/v1/user/data-bank`

Request Header:
- Authorized token (Cookies)

Request Body:
```json
{
    "question_rdn": 6,
    "bank_name": "Bank BCA",
    "bank_account_holder": "Slamet",
    "bank_account_number": "123456789",
    "bank_branch": "Jakarta"
}
```

Response Body(200):
```json
{
    "result": true,
    "message": "Save CIF Stage 1 Success"
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Invalid request",
    "data": "ID Card is required, .etc"
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "Please check connection server"
}
```