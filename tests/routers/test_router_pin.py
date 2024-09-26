from fastapi import FastAPI, status
from httpx import AsyncClient
from sqlalchemy.ext.asyncio import AsyncSession


async def test_pin_listing_when_no_created_returns_empty_list(
    fastapi_app: FastAPI,
    client: AsyncClient,
    dbsession: AsyncSession,
) -> None:
    url = fastapi_app.url_path_for("get_all_pins")
    response = await client.get(url)

    assert response.status_code == status.HTTP_200_OK

    data = response.json()

    assert data == []


async def test_pin_create_one_returns_valid_response(
    fastapi_app: FastAPI,
    client: AsyncClient,
    dbsession: AsyncSession,
) -> None:
    url = fastapi_app.url_path_for("create_pin")
    new_pin = {
        "server_id": 1000,
        "channel_id": 2000,
        "message_id": 3000,
        "user_id": 4000,
    }
    response = await client.post(url, json=new_pin)
    assert response.status_code == status.HTTP_201_CREATED


async def test_pin_create_single_can_be_listed(
    fastapi_app: FastAPI,
    client: AsyncClient,
    dbsession: AsyncSession,
) -> None:
    url = fastapi_app.url_path_for("create_pin")
    new_pin = {
        "server_id": 1000,
        "channel_id": 2000,
        "message_id": 3000,
        "user_id": 4000,
    }
    pin_response = await client.post(url, json=new_pin)
    assert pin_response.status_code == status.HTTP_201_CREATED

    url = fastapi_app.url_path_for("get_all_pins")
    all_pins_response = await client.get(url)

    assert all_pins_response.status_code == status.HTTP_200_OK

    created_pin = all_pins_response.json()[0]

    for k, v in new_pin.items():
        assert created_pin[k] == v


async def test_pin_create_duplicate_throws_409(
    fastapi_app: FastAPI,
    client: AsyncClient,
    dbsession: AsyncSession,
) -> None:
    url = fastapi_app.url_path_for("create_pin")
    new_pin = {
        "server_id": 1000,
        "channel_id": 2000,
        "message_id": 3000,
        "user_id": 4000,
    }
    response = await client.post(url, json=new_pin)
    assert response.status_code == status.HTTP_201_CREATED

    response = await client.post(url, json=new_pin)
    assert response.status_code == status.HTTP_409_CONFLICT


async def test_pin_remove_invalid_throws_404(
    fastapi_app: FastAPI,
    client: AsyncClient,
    dbsession: AsyncSession,
) -> None:
    url = fastapi_app.url_path_for(
        "remove_pin",
        server_id=1000,
        channel_id=2000,
        message_id=3000,
    )
    response = await client.delete(url)

    assert response.status_code == status.HTTP_404_NOT_FOUND


async def test_pin_create_and_delete_single_pin_lists_nothing(
    fastapi_app: FastAPI,
    client: AsyncClient,
    dbsession: AsyncSession,
) -> None:
    url = fastapi_app.url_path_for("create_pin")
    new_pin = {
        "server_id": 1000,
        "channel_id": 2000,
        "message_id": 3000,
        "user_id": 4000,
    }
    response = await client.post(url, json=new_pin)
    assert response.status_code == status.HTTP_201_CREATED

    url = fastapi_app.url_path_for(
        "remove_pin",
        server_id=1000,
        channel_id=2000,
        message_id=3000,
    )
    response = await client.delete(url)

    assert response.status_code == status.HTTP_204_NO_CONTENT

    url = fastapi_app.url_path_for("get_all_pins")
    response = await client.get(url)

    assert response.status_code == status.HTTP_200_OK

    data = response.json()

    assert data == []
