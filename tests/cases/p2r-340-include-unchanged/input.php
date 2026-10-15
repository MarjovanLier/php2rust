<?php

declare(strict_types=1);

final class Box
{
    public function __construct(public readonly int $size)
    {
    }

    public function doubled(): int
    {
        return $this->size * 2;
    }
}

function found(string $text): string
{
    return str_contains($text, 'b') ? 'yes' : 'no';
}

$box = new Box(21);
echo $box->doubled(), ' ', found('abc'), ' ', found('xyz'), "\n";
