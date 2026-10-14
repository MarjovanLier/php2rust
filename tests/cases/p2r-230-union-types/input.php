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

/** @param array-key $k */
function keyType(int|string $k): string
{
    return gettype($k);
}

/** @param 1|2|'a' $x */
function literal(int|string $x): string
{
    return gettype($x);
}

function flag(true|string $x): string
{
    return gettype($x);
}

function defaulted(int|string $v = 5): string
{
    return gettype($v);
}

/**
 * @implements ArrayAccess<int|string, string>
 */
final readonly class Lookup implements ArrayAccess
{
    public function offsetExists(mixed $offset): bool
    {
        return true;
    }

    public function offsetGet(mixed $offset): string
    {
        return gettype($offset);
    }

    public function offsetSet(mixed $offset, mixed $value): void {}

    public function offsetUnset(mixed $offset): void {}
}

final readonly class Tree
{
    /**
     * @param list<Tree>|int $kids
     */
    public function __construct(public array|int $kids) {}

    public function kind(): string
    {
        $kids = $this->kids;
        return gettype($kids);
    }
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
echo keyType(1), ' ', literal('a'), ' ', flag(true), ' ', flag('t'), "\n";
echo defaulted(), ' ', defaulted('x'), "\n";
$lookup = new Lookup();
echo $lookup->offsetGet(3), ' ', $lookup->offsetGet('k'), "\n";
$leaves = [new Tree(2)];
echo (new Tree(1))->kind(), ' ', (new Tree($leaves))->kind(), "\n";
