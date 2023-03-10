from datetime import datetime
from optparse import Option
from typing import Optional, Type, List
from pydantic import BaseModel, NoneBytes
from enum import Enum

class Token(BaseModel):
    access_token: str
    token_type: str

class TokenData(BaseModel):
    username: Optional[str] = None

class UserBase(BaseModel):
    username: str
    name: str

class UserCreate(UserBase):
    password: str
    
class UserUpdate(BaseModel):
    username: Optional[str] = None
    password: Optional[str] = None
    name: Optional[str] = None

class User(UserBase):
    id: int
    class Config:
        orm_mode = True

class ShiftCreate(BaseModel):
    name: str
    description:str

class Shift(ShiftCreate):
    id: int
    class Config:
        orm_mode = True

class ShiftEntryCreate(BaseModel):
    time_clock_in: datetime
    time_clock_out: datetime
    description: str

class ShiftEntry(ShiftEntryCreate):
    id: int
    class Config:
        orm_mode = True
