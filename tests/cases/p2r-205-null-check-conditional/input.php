<?php

declare(strict_types=1);

final readonly class Date
{
    public function __construct(
        public string $day,
    ) {}

    public static function make(string $text): self
    {
        return new self($text);
    }
}

final readonly class Holder
{
    public function __construct(
        public ?Date $date,
    ) {}
}

function plusOne(?int $x): int
{
    return $x !== null ? $x + 1 : 0;
}

function orZero(?int $x): int
{
    return $x === null ? 0 : $x * 2;
}

function label(?string $s): string
{
    return null !== $s ? $s : 'none';
}

function wrap(?string $text): ?Date
{
    return $text !== null ? Date::make($text) : null;
}

function show(?Date $d): void
{
    echo $d === null ? 'none' : $d->day, "\n";
}

function dayOf(Date $d): string
{
    return $d->day;
}

function pick(int $n): ?string
{
    if ($n > 0) {
        return '0' . $n;
    }
    return null;
}

$p = pick(3);
$h = new Holder($p !== null ? Date::make($p) : null);
$q = pick(0);
$g = new Holder($q !== null ? Date::make($q) : null);
show($h->date);
show($g->date);
echo plusOne(4), ' ', plusOne(null), ' ', orZero(5), ' ', orZero(null), "\n";
echo label('a'), label(null), "\n";
show(wrap('09'));
show(wrap(null));
echo dayOf($q === null ? new Date('00') : Date::make($q)), "\n";
echo dayOf($p === null ? new Date('00') : Date::make($p)), "\n";
$d = new Date('12');
$e = $p !== null ? $d : new Date('01');
echo $e->day, "\n";
$m = $h->date;
$text = $m === null ? 'none' : $m->day;
echo $text, "\n";
