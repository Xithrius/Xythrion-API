from datetime import datetime
from uuid import UUID

from pydantic import BaseModel


class LinkMapChannelCreate(BaseModel):
    server_id: int
    input_channel_id: int
    output_channel_id: int


class LinkMapChannelUpdate(BaseModel):
    pass


class LinkMapChannel(BaseModel):
    id: UUID
    created_at: datetime
    server_id: int
    input_channel_id: int
    output_channel_id: int


class LinkMapConverterBase(BaseModel):
    from_link: str
    to_link: str | None = None
    xpath: str | None = None


class LinkMapConverterCreate(LinkMapConverterBase): ...


class LinkMapConverterUpdate(BaseModel):
    pass


class LinkMapConverter(LinkMapConverterBase):
    id: UUID
    created_at: datetime


class LinkMapChannelConverters(LinkMapChannel):
    converters: list[LinkMapConverter]


class LinkMapConverterChannels(LinkMapConverter):
    channels: list[LinkMapChannel]
