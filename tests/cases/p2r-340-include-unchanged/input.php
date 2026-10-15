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

function year(string $text): string
{
    $date = DateTime::createFromFormat('!Y', $text);
    if ($date === false) {
        return 'none';
    }
    return $date->format('Y');
}

$box = new Box(21);
echo $box->doubled(), ' ', found('abc'), ' ', found('xyz'), ' ', strlen('four') + 1, ' ', year('1980'), "\n";
