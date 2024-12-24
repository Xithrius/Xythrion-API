from collections.abc import Iterable, Sequence
from typing import Any, Generic, TypeVar
from uuid import UUID

from pydantic import BaseModel
from sqlalchemy import ColumnExpressionArgument, delete, select, true, update
from sqlalchemy.ext.asyncio import AsyncSession

from api.database.base import Base

ModelType = TypeVar("ModelType", bound=Base)
CreateSchemaType = TypeVar("CreateSchemaType", bound=BaseModel)
UpdateSchemaType = TypeVar("UpdateSchemaType", bound=BaseModel)

type Filters = ColumnExpressionArgument[bool]
type PrimaryKeyIDType = int | str | UUID

EMPTY_FILTERS = true()

# Modified verison of the CRUDPlus class
# https://github.com/fastapi-practices/sqlalchemy-crud-plus/blob/master/sqlalchemy_crud_plus/crud.py
# 114c7bd004be2afc8d529cd52403250a1041bbdd


class CRUDBase(Generic[ModelType, CreateSchemaType, UpdateSchemaType]):
    def __init__(self, model: type[ModelType]) -> None:
        self.model = model

    async def create_(
        self,
        session: AsyncSession,
        *,
        obj: CreateSchemaType,
        extra: dict[str, Any] | None = None,
        commit: bool = True,
    ) -> ModelType:
        if extra is None:
            extra = {}

        ins = self.model(**obj.model_dump()) if not extra else self.model(**obj.model_dump(), **extra)
        session.add(ins)

        if commit:
            await session.commit()

        return ins

    async def create_all_(
        self,
        session: AsyncSession,
        *,
        obj: Iterable[CreateSchemaType],
        commit: bool = True,
    ) -> list[ModelType]:
        ins_list: list[ModelType] = []

        for ins in obj:
            ins_list.append(self.model(**ins.model_dump()))

        session.add_all(ins_list)

        if commit:
            await session.commit()

        return ins_list

    async def select_(
        self,
        session: AsyncSession,
        *,
        filters: Filters = EMPTY_FILTERS,
        limit: int | None = None,
        offset: int | None = None,
    ) -> Sequence[ModelType]:
        stmt = select(self.model).where(filters).limit(limit).offset(offset)
        query = await session.execute(stmt)

        return query.scalars().all()

    async def select_first_(
        self,
        session: AsyncSession,
        *,
        filters: Filters = EMPTY_FILTERS,
    ) -> ModelType | None:
        stmt = select(self.model).where(filters)
        query = await session.execute(stmt)

        return query.scalars().first()

    async def select_by_id(
        self,
        session: AsyncSession,
        *,
        pk: PrimaryKeyIDType,
    ) -> ModelType | None:
        rows = await self.select_first_(
            session,
            filters=self.model.id == pk,  # pyright: ignore[reportUnknownArgumentType, reportAttributeAccessIssue]
        )

        return rows

    async def select_all(
        self,
        session: AsyncSession,
        *,
        limit: int | None = None,
        offset: int | None = None,
    ) -> Sequence[ModelType]:
        rows = await self.select_(
            session,
            limit=limit,
            offset=offset,
        )

        return rows

    async def update_(
        self,
        session: AsyncSession,
        *,
        filters: Filters = EMPTY_FILTERS,
        obj: UpdateSchemaType | dict[str, Any],
        commit: bool = False,
    ) -> int:
        instance_data = obj if isinstance(obj, dict) else obj.model_dump(exclude_unset=True)
        stmt = update(self.model).where(filters).values(**instance_data)

        result = await session.execute(stmt)

        if commit:
            await session.commit()

        return result.rowcount

    async def update_by_id(
        self,
        session: AsyncSession,
        *,
        pk: PrimaryKeyIDType,
        obj: UpdateSchemaType | dict[str, Any],
        commit: bool = False,
    ) -> int:
        rowcount = await self.update_(
            session,
            filters=self.model.id == pk,  # pyright: ignore[reportUnknownArgumentType, reportAttributeAccessIssue]
            obj=obj,
            commit=commit,
        )

        return rowcount

    async def delete_(
        self,
        session: AsyncSession,
        *,
        filters: Filters = EMPTY_FILTERS,
        logical_deletion: bool = False,
        delete_flag: str = "del_flag",
        commit: bool = False,
    ) -> int:
        if logical_deletion:
            flags = {delete_flag: True}
            stmt = update(self.model).where(filters).values(**flags)
        else:
            stmt = delete(self.model).where(filters)

        result = await session.execute(stmt)

        if commit:
            await session.commit()

        return result.rowcount

    async def delete_by_id(
        self,
        session: AsyncSession,
        *,
        pk: PrimaryKeyIDType,
        logical_deletion: bool = False,
        commit: bool = False,
    ) -> int:
        rowcount = await self.delete_(
            session,
            filters=self.model.id == pk,  # pyright: ignore[reportUnknownArgumentType, reportAttributeAccessIssue]
            logical_deletion=logical_deletion,
            commit=commit,
        )

        return rowcount
