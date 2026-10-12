<?php

declare(strict_types=1);

enum Suit: string
{
    case Hearts = 'H';
    case Spades = 'S';
}

enum Level: int
{
    case Low = -1;
    case High = 0x10;
}

enum Status
{
    case ACTIVE;
    case IN_REVIEW;
}

function describe(Suit $s): string
{
    return $s->name . '=' . $s->value . "\n";
}

function code(Level $l): int
{
    return $l->value * 2;
}

$s = Suit::from('S');
echo describe($s), describe(Suit::Hearts);
$v = 'H';
$h = Suit::from($v);
echo $h->name, "\n";
$x = Suit::tryFrom('X');
if ($x === null) {
    echo "none\n";
}
$y = Suit::tryFrom('H') ?? Suit::Spades;
echo $y->name, "\n";
echo code(Level::from(16)), ' ', code(Level::Low), "\n";
echo count(Status::cases()), "\n";
foreach (Suit::cases() as $c) {
    echo $c->value;
}
echo "\n";
$name = Status::IN_REVIEW->name;
echo $name, "\n";
