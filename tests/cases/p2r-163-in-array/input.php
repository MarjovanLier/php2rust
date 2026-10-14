<?php

declare(strict_types=1);

enum Suit
{
    case Hearts;
    case Diamonds;
    case Spades;
}

final readonly class Card
{
    public function __construct(public string $rank, public Suit $suit) {}

    public function face(): bool
    {
        return in_array($this->rank, ['J', 'Q', 'K'], true);
    }

    public function red(): bool
    {
        return in_array($this->suit, [Suit::Hearts, Suit::Diamonds], true);
    }
}

function small(int $n): bool
{
    return in_array($n, [-2, -1, 0, 1, 2], true) || in_array($n * 10, [100], true);
}

function known(string $code): bool
{
    $upper = $code . "!";
    return in_array($code, ["a", "b"], true) || in_array($upper, ["c!"], true);
}

$card = new Card("Q", Suit::Hearts);
if ($card->face() && $card->red() && !(new Card("2", Suit::Spades))->red()) {
    echo "red face\n";
}
if (small(-2) && small(10) && !small(3) && known("c") && !known("d")) {
    echo "small and known\n";
}
if (in_array(3000000000, [1, 3000000000], true)) {
    echo "big\n";
}
