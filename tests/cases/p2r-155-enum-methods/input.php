<?php

declare(strict_types=1);

final class Tally
{
    public int $total = 0;
}

enum Suit: string
{
    case Hearts = 'h';
    case Spades = 's';

    public function label(): string
    {
        return match ($this) {
            self::Hearts => 'hearts',
            self::Spades => 'spades',
        };
    }

    public function shout(): string
    {
        return $this->label() . '!' . $this->value;
    }

    public function score(Tally $tally): void
    {
        if ($this === Suit::Hearts) {
            $tally->total += 10;
        } else {
            $tally->total += 1;
        }
    }
}

$suit = Suit::Spades;
echo $suit->shout(), "\n";
echo Suit::Hearts->label(), "\n";
$tally = new Tally();
Suit::Hearts->score($tally);
$suit->score($tally);
echo $tally->total, "\n";
