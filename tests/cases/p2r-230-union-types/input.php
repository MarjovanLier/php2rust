<?php

declare(strict_types=1);

enum Suit: string
{
    case Hearts = 'hearts';
    case Spades = 'spades';
}

final readonly class Card
{
    public function __construct(
        public int|string $rank,
        public Suit|bool|null $suit,
    ) {}

    public function rank(): int|string
    {
        return $this->rank;
    }
}

function describe(int|string $v): string
{
    return gettype($v);
}

function pick(bool $b): int|string
{
    if ($b) {
        return 5;
    }
    return 'five';
}

/**
 * @param array{x: int, y: int}|list<int>|scalar|null $v
 */
function kind(array|int|float|string|bool|null $v): string
{
    return gettype($v);
}

/**
 * @return array{x: int, y: int}|list<int>
 */
function point(bool $flat): array
{
    if ($flat) {
        $list = [1, 2];
        return $list;
    }
    $shape = ['x' => 1, 'y' => 2];
    return $shape;
}

/**
 * @param array{x: int, y: int}|list<int> $v
 */
function pointKind(array $v): string
{
    return gettype($v);
}

function suited(Card $card): string
{
    $suit = $card->suit;
    return gettype($suit);
}

echo describe(1), ' ', describe('a'), "\n";
$u = pick(true);
echo describe($u), ' ', describe(pick(false)), "\n";
echo kind(1), ' ', kind(1.5), ' ', kind(true), ' ', kind('s'), ' ', kind(null), "\n";
echo pointKind(point(true)), ' ', pointKind(point(false)), "\n";
$card = new Card('Q', Suit::Hearts);
$joker = new Card(0, false);
echo describe($card->rank()), ' ', describe($joker->rank()), "\n";
echo suited($card), ' ', suited($joker), ' ', suited(new Card(2, null)), "\n";
