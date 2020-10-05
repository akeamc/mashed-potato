# 🥔

## API reference

### `Dish` object

| Field      | Type   |
| ---------- | ------ |
| `title`    | string |
| `id`       | string |
| `co2e_url` | string |
| `co2e`     | `f64`  |

### `Menu` object

| Field    | Type                     |
| -------- | ------------------------ |
| `date`   | ISO8601 timestamp        |
| `dishes` | [`Dish[]`](#dish-object) |

### `Study set` object

| Field         | Type   |
| ------------- | ------ |
| `id`          | string |
| `title`       | string |
| `description` | string |
| `term_count`  | `u32`  |
| `author`      | string |

### `GET /menu`

Returns an array of [`Menu` objects](#menu-object).

### `GET /dishes`

Returns an array of [`Dish` objects](#dish-object).

### `GET /dishes/:dish`

Returns a single [`Dish` object](#dish-object) with the specified id, if a dish with such id exists.

### `GET /quizlet/:id`

Returns a [`Study set` object](#study-set-object) with the specified id, given that a study set with such id exists.