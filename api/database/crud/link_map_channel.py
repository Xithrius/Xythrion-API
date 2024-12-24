from collections.abc import Sequence

from sqlalchemy import and_, exists, select
from sqlalchemy.ext.asyncio import AsyncSession

from api.database.crud.base import CRUDBase
from api.database.models.link_map import (
    LinkMapChannelConverterAssociationModel,
    LinkMapChannelModel,
    LinkMapConverterModel,
)
from api.routers.schemas.link_map import LinkMapChannelCreate, LinkMapChannelUpdate


class LinkMapChannelCRUD(CRUDBase[LinkMapChannelModel, LinkMapChannelCreate, LinkMapChannelUpdate]):
    async def get_all(self, db: AsyncSession) -> Sequence[LinkMapChannelModel]:
        items = await db.execute(select(self.model))
        items.unique()

        return items.scalars().all()

    async def get_by_server_id(self, db: AsyncSession, *, server_id: int) -> Sequence[LinkMapChannelModel]:
        items = await db.execute(select(self.model).where(self.model.server_id == server_id))
        items.unique()

        return items.scalars().all()

    async def get_converters_for_channel(
        self,
        db: AsyncSession,
        *,
        input_channel_id: int,
    ) -> list[LinkMapConverterModel] | None:
        items = await db.execute(select(self.model).where(self.model.input_channel_id == input_channel_id))
        items.unique()

        if (channel_converters := items.scalars().first()) is not None:
            return channel_converters.converters

        return None

    async def create(self, db: AsyncSession, *, obj: LinkMapChannelCreate) -> LinkMapChannelModel:
        return await self.create_(db, obj=obj)

    async def add_converter(self, db: AsyncSession, *, channel_id: str, converter_id: str) -> str | None:
        # Assuming that the channel and converter were already checked to exist beforehand

        association_exists = await db.scalar(
            select(
                exists(
                    and_(
                        LinkMapChannelConverterAssociationModel.channel_id == channel_id,
                        LinkMapChannelConverterAssociationModel.converter_id == converter_id,
                    ),
                ),
            ),
        )

        if association_exists:
            return None

        channel_results = await db.execute(select(self.model).where(self.model.id == channel_id))
        channel = channel_results.scalars().first()

        converter_results = await db.execute(
            select(LinkMapConverterModel).where(LinkMapConverterModel.id == converter_id),
        )
        converter = converter_results.scalars().first()

        channel.converters.append(converter)  # pyright: ignore[reportArgumentType, reportOptionalMemberAccess]

        if converter:
            return str(converter.id)

        return None

    async def remove_converter(self, db: AsyncSession, *, channel_id: str, converter_id: str) -> None:
        # Assuming that the channel and converter were already checked to exist beforehand

        channel_results = await db.execute(select(self.model).where(self.model.id == channel_id))
        channel = channel_results.scalars().first()

        converter_results = await db.execute(
            select(LinkMapConverterModel).where(LinkMapConverterModel.id == converter_id),
        )
        converter = converter_results.scalars().first()

        channel.converters.remove(converter)  # pyright: ignore[reportArgumentType, reportOptionalMemberAccess]

    async def remove_children(self, db: AsyncSession, *, pk: str) -> LinkMapConverterModel | None:
        item = await db.get(self.model, pk)

        if (channel := item) is None:
            return None

        channel.converters = []

        return channel  # pyright: ignore[reportReturnType]

    async def delete(self, db: AsyncSession, *, pk: str, cascade_once: bool = False) -> int:
        if cascade_once:
            converter = await self.remove_children(db, pk=pk)

            if converter is None:
                return 0

        return await self.delete_by_id(db, pk=pk)


link_map_channel_dao = LinkMapChannelCRUD(LinkMapChannelModel)
