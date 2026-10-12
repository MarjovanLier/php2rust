<?php

declare(strict_types=1);

enum Suit: string
{
    case Hearts = 'H';
    case Spades = 'S';
}

enum Status
{
    case ACTIVE;
    case IN_REVIEW;
    case done;
}

function color(Suit $s): string
{
    if ($s === Suit::Hearts) {
        return "red\n";
    }
    return "black\n";
}

function advance(Status $s): Status
{
    if ($s == Status::ACTIVE) {
        return Status::IN_REVIEW;
    }
    return Status::done;
}

function label(?Status $s): string
{
    if ($s === null) {
        return "none\n";
    }
    if ($s !== Status::done) {
        return "open\n";
    }
    return "closed\n";
}

/** @param list<Suit> $suits */
function reds(array $suits): int
{
    $count = 0;
    foreach ($suits as $s) {
        if ($s === Suit::Hearts) {
            $count += 1;
        }
    }
    return $count;
}

$s = Suit::Spades;
echo color($s), color(suit::Hearts);
echo label(advance(Status::ACTIVE)), label(advance(Status::IN_REVIEW)), label(null);
echo reds([Suit::Hearts, $s, Suit::Hearts]), "\n";
$t = $s;
if ($t === Suit::Spades) {
    echo "same\n";
}
