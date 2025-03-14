# Option List Api Specification

## Get List Nationality
Endpoint: **GET** `/api/v1/option/nationality`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "Country list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "AX",
            "description": "AALAND ISLANDS"
        },
        {
            "data_id": 2,
            "code": "ID",
            "description": "Indonesian"
        },
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List City (Kota)
Endpoint: **GET** `/api/v1/option/city/{query string}`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "Cities list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "Prov. Bali",
            "description": "Kab. Jembrana"
        },
        {
            "data_id": 107,
            "code": "Prov. Jawa Timur",
            "description": "Kab. Jember"
        }
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List District(Kecamatan)
Endpoint: **GET** `/api/v1/option/district/{city data_id}`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "District list retrieved successfully",
    "data": [
        {
            "code": "Jembrana"
        },
        {
            "code": "Melaya"
        },
        {
            "code": "Mendoyo"
        },
        ```````````
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List subdistrict(Kelurahan)
Endpoint: **GET** `/api/v1/option/subdistrict/{district code}`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "Sub District list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "Gilimanuk",
            "description": "82252"
        },
        {
            "data_id": 2,
            "code": "Melaya",
            "description": "82252"
        },
        {
            "data_id": 3,
            "code": "Belimbingsari",
            "description": "82252"
        },
        `````````````
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Sales
Endpoint: **GET** `/api/v1/option/sales`

Response Body(200):
```json
{
    "result": true,
    "message": "Sales list retrieved successfully",
    "data": [
        {
            "data_id": 24,
            "code": "RM",
            "description": "Risk Management"
        },
        {
            "data_id": 2708,
            "code": "RIA",
            "description": "PARIAMA SIANIPAR"
        },
        {
            "data_id": 3276,
            "code": "VEGA",
            "description": "VEGA ROSINTAN"
        },
        ````````````
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Bank
Endpoint: **GET** `/api/v1/option/bank`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "Bank list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "BBRI",
            "description": "PT. BANK RAKYAT INDONESIA (PERSERO), Tbk"
        },
        {
            "data_id": 2,
            "code": "BMRI",
            "description": "BANK MANDIRI"
        },
        {
            "data_id": 3,
            "code": "BBNI",
            "description": "PT. BANK NEGARA INDONESIA (PERSERO),Tbk"
        },
        ```````````
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List NPWP
Endpoint: **GET** `/api/v1/option/npwp`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "Question NPWP list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "Memiliki NPWP",
            "description": ""
        },
        {
            "data_id": 2,
            "code": "Tidak memiliki NPWP - Ikut pasangan",
            "description": ""
        },
        {
            "data_id": 3,
            "code": "Tidak memiliki NPWP - Belum bekerja",
            "description": ""
        },
        {
            "data_id": 4,
            "code": "Tidak memiliki NPWP - Alasan lainnya",
            "description": ""
        }
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Income
Endpoint: **GET** `/api/v1/option/income`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "IncomePerAnnum list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "< Rp 50 juta",
            "description": "< Rp 50 juta"
        },
        {
            "data_id": 2,
            "code": "Rp.50 juta - 100 juta",
            "description": "Rp.50 juta - 100 juta"
        },
        {
            "data_id": 3,
            "code": "Rp.100 juta - 500 juta",
            "description": "Rp.100 juta - 500 juta"
        },
        ```````````
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Education
Endpoint: **GET** `/api/v1/option/education`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "Educational list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "OTH : Others",
            "description": "OTH : Others"
        },
        {
            "data_id": 2,
            "code": "SD : Elementary School",
            "description": "SD : Elementary School"
        },
        {
            "data_id": 3,
            "code": "SMP : Junior High School",
            "description": "SMP : Junior High School"
        },
        ```````````
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Marital Status
Endpoint: **GET** `/api/v1/option/maritalstatus`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "MaritalStatus list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "Single",
            "description": "Single"
        },
        {
            "data_id": 2,
            "code": "Married",
            "description": "Married"
        },
        {
            "data_id": 3,
            "code": "Widower",
            "description": "Widower"
        }
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Gender
Endpoint: **GET** `/api/v1/option/gender`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "Sex list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "Male",
            "description": "Male"
        },
        {
            "data_id": 2,
            "code": "Female",
            "description": "Female"
        }
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Religion
Endpoint: **GET** `/api/v1/option/religion`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "Religion list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "ISLAM",
            "description": "ISLAM"
        },
        {
            "data_id": 2,
            "code": "PROTESTAN",
            "description": "PROTESTAN"
        },
        {
            "data_id": 3,
            "code": "KATHOLIK",
            "description": "KATHOLIK"
        },
        ```````````
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Fund Source
Endpoint: **GET** `/api/v1/option/fundsource`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "FundSource list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "Others",
            "description": "Others"
        },
        {
            "data_id": 2,
            "code": "Salary",
            "description": "Salary"
        },
        {
            "data_id": 3,
            "code": "Business Profit",
            "description": "Business Profit"
        },
        ```````````
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Residence Status
Endpoint: **GET** `/api/v1/option/residencestatus`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "ResidencyStatus list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "Milik Sendiri",
            "description": "Milik Sendiri"
        },
        {
            "data_id": 2,
            "code": "Milik Keluarga",
            "description": "Milik Keluarga"
        },
        {
            "data_id": 3,
            "code": "Milik Perusahaan",
            "description": "Milik Perusahaan"
        },
        ```````````
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Beneficiary Ownership
Endpoint: **GET** `/api/v1/option/beneficiary`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "AssetOwner list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "MYSELF",
            "description": "MY SELF"
        },
        {
            "data_id": 2,
            "code": "BENEFICIARY OWNER",
            "description": "BENEFICIARY OWNER"
        }
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Investment Objective
Endpoint: **GET** `/api/v1/option/investmentobjective`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "InvestmentObjectives list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "Others",
            "description": "Others"
        },
        {
            "data_id": 2,
            "code": "Price Appreciation",
            "description": "Price Appreciation"
        },
        {
            "data_id": 3,
            "code": "Long Term Investment",
            "description": "Long Term Investment"
        },
        {
            "data_id": 4,
            "code": "Speculation",
            "description": "Speculation"
        },
        {
            "data_id": 5,
            "code": "Income",
            "description": "Income"
        }
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Risk
Endpoint: **GET** `/api/v1/option/risk`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "Risk list retrieved successfully",
    "data": [
        {
            "data_id": 0,
            "code": "LOW RISK",
            "description": "LOW RISK"
        },
        {
            "data_id": 1,
            "code": "MEDIUM RISK",
            "description": "MEDIUM RISK"
        },
        {
            "data_id": 2,
            "code": "HIGH RISK",
            "description": "HIGH RISK"
        }
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Pekerjaan
Endpoint: **GET** `/api/v1/option/occupation`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "Occupation list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "Pelajar/Mahasiswa",
            "description": "Pelajar/Mahasiswa"
        },
        {
            "data_id": 2,
            "code": "Ibu Rumah Tangga",
            "description": "Ibu Rumah Tangga"
        },
        {
            "data_id": 3,
            "code": "Karyawan Swasta",
            "description": "Karyawan Swasta"
        },
        ```````````
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Position
Endpoint: **GET** `/api/v1/option/position/{occupation_id}`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "Sub District list retrieved successfully",
    "data": [
        {
            "data_id": 0,
            "code": "Lainnya",
            "description": "Lainnya"
        },
        {
            "data_id": 2,
            "code": "PNS Esselon 1",
            "description": "PNS Esselon 1"
        },
        {
            "data_id": 3,
            "code": "PNS Esselon 2",
            "description": "PNS Esselon 2"
        },
        ```````````
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Nature of Business
Endpoint: **GET** `/api/v1/option/naturebusiness/{occupation_id}/{position_id}`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "Nature of bussiness list retrieved successfully",
    "data": [
        {
            "data_id": 2,
            "code": "Pemerintahan",
            "description": "Pemerintahan"
        }
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Spouse Relationship
Endpoint: **GET** `/api/v1/option/spouse-relationship`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "SpouseRelationship list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "OTHERS",
            "description": "OTHERS"
        },
        {
            "data_id": 2,
            "code": "IBU",
            "description": "IBU"
        },
        {
            "data_id": 3,
            "code": "AYAH",
            "description": "AYAH"
        },
        {
            "data_id": 4,
            "code": "ISTRI",
            "description": "ISTRI"
        },
        {
            "data_id": 5,
            "code": "SUAMI",
            "description": "SUAMI"
        }
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Spouse Occupation
Endpoint: **GET** `/api/v1/option/spouse-occupation`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "SpouseOccupation list retrieved successfully",
    "data": [
        {
            "data_id": 2,
            "code": "Private Employee ( Pegawai Swasta )",
            "description": "Private Employee ( Pegawai Swasta )"
        },
        {
            "data_id": 3,
            "code": "Civil Servant ( Pegawai Negeri )",
            "description": "Civil Servant ( Pegawai Negeri )"
        },
        {
            "data_id": 5,
            "code": "Entrepeneur ( Pengusaha )",
            "description": "Entrepeneur ( Pengusaha )"
        },
        {
            "data_id": 7,
            "code": "TNI / POLICE ( TNI / Polisi )",
            "description": "TNI / POLICE ( TNI / Polisi )"
        },
        {
            "data_id": 8,
            "code": "Retirement ( Pensiunan )",
            "description": "Retirement ( Pensiunan )"
        },
        {
            "data_id": 9,
            "code": "Teacher ( Guru )",
            "description": "Teacher ( Guru )"
        }
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Spouse Position
Endpoint: **GET** `/api/v1/option/spouse-position`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "Position list retrieved successfully",
    "data": [
        {
            "data_id": 0,
            "code": "Default (Nasabah Non High Risk)",
            "description": "Default (Nasabah Non High Risk)"
        },
        {
            "data_id": 1,
            "code": "Presiden",
            "description": "Presiden"
        },
        {
            "data_id": 2,
            "code": "Wakil Presiden",
            "description": "Wakil Presiden"
        },
        {
            "data_id": 3,
            "code": "Pejabat setingkat Menteri",
            "description": "Pejabat Setingkat Menteri"
        },
        ````````````
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Spouse Nature Of Business
Endpoint: **GET** `/api/v1/option/spouse-naturebusiness`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "NatureOfBusiness list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "Money Changer, Money Remittance",
            "description": "Money Changer, Money Remittance"
        },
        {
            "data_id": 2,
            "code": "Offshore Companies",
            "description": "Offshore Companies"
        },
        {
            "data_id": 3,
            "code": "Car Dealer",
            "description": "Car Dealer"
        },
        ````````````
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Bank RDN
Endpoint: **GET** `/api/v1/option/bank-rdn`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "QuestionRDN list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "BCA (ONLINE)",
            "description": "BCA (ONLINE)"
        },
        {
            "data_id": 2,
            "code": "BRI (OFFLINE)",
            "description": "BRI (OFFLINE)"
        },
        {
            "data_id": 3,
            "code": "MANDIRI (OFFLINE)",
            "description": "MANDIRI (OFFLINE)"
        },
        {
            "data_id": 4,
            "code": "CIMBNIAGA (OFFLINE)",
            "description": "CIMBNIAGA (OFFLINE)"
        },
        {
            "data_id": 5,
            "code": "MAYAPADA (OFFLINE)",
            "description": "MAYAPADA (OFFLINE)"
        }
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```

## Get List Client Category
Endpoint: **GET** `/api/v1/option/category`

Request Header:
- Authorized token (Cookies)

Response Body(200):
```json
{
    "result": true,
    "message": "ClientNCategory list retrieved successfully",
    "data": [
        {
            "data_id": 1,
            "code": "ONLINE",
            "description": "ONLINE"
        }
    ]
}
```

Response Body(400):
```json
{
    "result": false,
    "message": "Data not found",
    "data": [ ]
}
```

Response Body(500):
```json
{
    "result": false,
    "message": "Internal Server Error",
    "data": "ex: Invalid column name 'data_id'"
}
```
