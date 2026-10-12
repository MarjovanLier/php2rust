<?php

declare(strict_types=1);

enum Suit: string
{
    case Hearts = 'H';
    case Spades = 'S';
}

enum Nothing
{
}

enum Long
{
    case Zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz;
}

function show(Suit $e): string
{
    return $e->name . $e->name . "\n";
}

function never(Nothing $n): string
{
    return $n->name;
}

/** @param list<Suit> $xs */
function first(array $xs): string
{
    return $xs[0]->name . $xs[0]->name;
}

$e = Suit::Spades;
echo $e->name, $e->name, "\n";
echo show(Suit::Hearts);
echo first([Suit::Spades]), "\n";
echo $e->value . $e->value, "\n";
echo count(Long::cases()), "\n";
