# Cymer API

!!
**All interfaces and enums are defined at the end of this file in the `Resources` section**

All response are wrapped into this interface:
```typescript
interface CymerResponse {
  message: string; // Contains a useful message
  data: null|any; // Contains the data you requested
  code: ResponseCode; // A code is always provided
}
```

## Session

!!
**`/login` and `/signup` endpoints return a `token` that you must include in your queries following JWT Bearer protocol**

### Login
`POST /login`

```typescript
interface Input {
  email: string;
  password: string;
}

interface Output {
  token: string; // Token to provide in Authorization header
  user: User;
}
```
### Sign up
`POST /login`

```typescript
interface Input {
  email: string;
  password: string;
}

interface Output {
  token: string; // Token to provide in Authorization header
  user: User;
}
```

## User

### Get
`GET /user/?{id}`

```typescript
Output: User;
```

### Update
`PUT /user/?{id}`

```typescript
Input: User;

Output: User;
```

### Get Notification
`GET /user/notification`

```typescript
Output: string[];
```

### Read Notification
`DELETE /user/notification`

```typescript
Output: null;
```

## Company

### Get Company
`GET /company/?{id}`

```typescript
Output: Company;
```

### Get Employees
`GET /company/?{id}/employees`

```typescript
Output: User[];
```

### List Available Companies
`GET /company/list`

```typescript
Output: Company[];
```

### Set Current Company
`POST /company/use/{id}`

```typescript
Input: {};

Output: Company;
```

### Create new Company
`POST /company`

!!
**The newly created company will automatically be set as `current company`**

```typescript
Input: Company;

Output: Company;
```

### Update Company
`PUT /company/?{id}`

```typescript
Input: Company;

Output: Company;
```

### Disable Company
`DELETE /company/id`

```typescript
Output: null
```

## Employee

### Invite
`POST /employee/invite`

```typescript
interface Input {
  email: string; // Email of the future employee.
  employee_access: EmployeeAccess[];
}

Output: null;
```

### Remove
`DELETE /employee/{id}`

```typescript
Output: null;
```


## References

### Access
```typescript
enum UserAccess {
  User = 1,
  Admin = 2,
}
```

### ResponseCode

```typescript
enum ResponseCode {
  Success,
  ResourceCreated,
  ServerError,
  DatabaseError,
  Unauthorized,
  InvalidPassword,
  InvalidInput,
  ResourceNotFound,
  ResourceAlreadyExisting,
  NotImplemented,
}
```

### User

```typescript
interface User {
  id: number;
  email: string;
}
```

### Company

```typescript
interface Company {
  id: number;
  label: string;
}
```