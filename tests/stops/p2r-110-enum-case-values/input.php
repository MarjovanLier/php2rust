<?php

declare(strict_types=1);

enum Level: int
{
    case Low;
    case High = 2;
}

enum Status
{
    case Active = 1;
}

#[Attribute]
enum Suit
{
    case Hearts;
    #[Deprecated]
    case Spades;
}

enum Ratio: float
{
    case Half = 1.5;
}
