# Authentication Api Specification

## Register User
Endpoint: **POST** `/api/v1/auth/register`

Request Body:
```json
{
  "email": "string",
  "password": "string",
  "mobile_phone": "string",
  "fullname": "string",
  "bank_account_number": "string",
  "bank_name": "string",
  "bank_account_holder": "string",
  "question_rdn": 0,
  "sales": 0,
  "referral": 0,
  "client_category": 0
}
```
Response Body(200):
```json
{
    "result": true,
    "message": "Register Success"
}
```
Response Body(400):
```json
{
    "result": false,
    "message": "Invalid Request",
    "error": "Email is required, .etc"
}
```
Response Body(500):
```json
{
    "result": false,
    "message": "Register Failed",
    "error": "Invalid cplumn name cookies"
}
```

## Login User
### Login with Email and Password
Endpoint: **POST** `/api/v1/auth/login`

Request Body:
```json
{
  "email": "string",
  "password": "string"
}
```
Response Body(200):
```json
{
    "result": true,
    "message": "Login Success",
    "data": {
        "email": "example@gmail.com",
        "fullname": "John Doe",
        "mobile_phone": "628123456789",
        "bank_account_number": "1234567890",
        "bank_name": "BNI",
        "bank_account_holder": "John Doe",
        "question_rdn": 1,
        "sales": 1,
        "referral": 1,
        "client_category": 1,
        "disabled": true,
        "picture": "string",
    }
}
```
Response Body(400):
```json
{
    "result": false,
    "message": "Invalid Request",
    "error": "Invalid email & password, .etc"
}
```
Response Body(500):
```json
{
    "result": false,
    "message": "Login Failed",
    "error": "Internal Server Error"
}
```
### Login with Google
Endpoint: **GET** `/api/v1/auth/google/login`

Request Body:
```json
{
    "email": "string",
    "fullname": "string",
    "mobile_phone": "string",
    "picture": "string",
}
```
Response Body(200):
```json
{
    "result": true,
    "message": "Login Success",
    "data": {
        "email": "example@gmail.com",
        "fullname": "John Doe",
        "mobile_phone": "628123456789",
        "bank_account_number": "1234567890",
        "bank_name": "BNI",
        "bank_account_holder": "John Doe",
        "question_rdn": null,
        "sales": null,
        "referral": null,
        "client_category": null,
        "disabled": true,
        "picture": "/api/v1/assets/profile-1234567890.png",
    }
}
```
Response Body(400):
```json
{
    "result": false,
    "message": "Invalid Request",
    "error": "Email already register, .etc"
}
```
Response Body(500):
```json
{
    "result": false,
    "message": "Login Failed",
    "error": "Internal Server Error"
}
```

## Session User
Endpoint: **GET** `/api/v1/auth/session`

Response Body(200):
```json
{
    "result": true,
    "message": "Session active",
    "data": {
        "email": "example@gmail.com",
        "fullname": "John Doe",
        "mobile_phone": "628123456789",
        "bank_account_number": "1234567890",
        "bank_name": "BNI",
        "bank_account_holder": "John Doe",
        "question_rdn": 1,
        "sales": 1,
        "referral": 1,
        "client_category": 1,
        "disabled": true,
        "picture": "/api/v1/assets/profile-1234567890.png",
    }
}
```
Response Body(400):
```json
{
    "result": false,
    "message": "Invalid Request",
    "error": "Token not found, .etc"
}
```
Response Body(500):
```json
{
    "result": false,
    "message": "Session Failed",
    "error": "Internal Server Error"
}
```

## Refresh Token
Endpoint: **GET** `/api/v1/auth/cookies`

Response Body(200):
```json
{
    "result": true,
    "message": "Refresh token success",
    "data": {
        "email": "example@gmail.com",
        "fullname": "John Doe",
        "mobile_phone": "628123456789",
        "bank_account_number": "1234567890",
        "bank_name": "BNI",
        "bank_account_holder": "John Doe",
        "question_rdn": 1,
        "sales": 1,
        "referral": 1,
        "client_category": 1,
        "disabled": true,
        "picture": "/api/v1/assets/profile-1234567890.png",
    }
}
```
Response Body(400):
```json
{
    "result": false,
    "message": "Invalid Request",
    "error": "Token not found, .etc"
}
```
Response Body(500):
```json
{
    "result": false,
    "message": "Refresh token failed",
    "error": "Internal Server Error"
}
```

## Activation User
Endpoint: **GET** `/api/v1/auth/activation?otp-token={token}`

Response Body(200):
```json
{
    "result": true,
    "message": "Activation Success"
}
```
Response Body(400):
```json
{
    "result": false,
    "message": "Activation Failed",
    "error": "Token is not valid, .etc"
}
```
Response Body(500):
```json
{
    "result": false,
    "message": "Activation Failed",
    "error": "Internal Server Error"
}
```

## Forgot Password
Endpoint: **POST** `/api/v1/auth/forgot-password`

Request Body:
```json
{
  "email": "string"
}
```
Response Body(200):
```json
{
    "result": true,
    "message": "Forgot password success"
}
```
Response Body(400):
```json
{
    "result": false,
    "message": "Forgot password failed",
    "error": "Email not found, .etc"
}
```
Response Body(500):
```json
{
    "result": false,
    "message": "Forgot password failed",
    "error": "Internal Server Error"
}
```

## Reset Password
Endpoint: **POST** `/api/v1/auth/reset-password`

Request Body:
```json
{
  "email": "string",
  "password": "string",
  "otp_token": "string"
}
```
Response Body(200):
```json
{
    "result": true,
    "message": "Reset password success"
}
```
Response Body(400):
```json
{
    "result": false,
    "message": "Reset password failed",
    "error": "OTP Token is not valid"
}
```
Response Body(500):
```json
{
    "result": false,
    "message": "Reset password failed",
    "error": "Internal Server Error"
}
```