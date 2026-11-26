<?php

declare(strict_types=1);

enum Suit: string
{
    case Hearts = 'H';

    public function label(): string
    {
        return 'Hearts';
    }
}

final readonly class Leaf
{
    public function __construct(public string $name) {}

    public function hello(): void {}
}

function label(?Suit $suit): ?string
{
    return $suit?->label();
}

function voidCall(?Leaf $leaf): void
{
    $done = $leaf?->hello();
    echo $done === null ? "none\n" : "some\n";
}

echo label(Suit::Hearts) ?? '', "\n";
voidCall(new Leaf('a'));
