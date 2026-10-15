<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Data;

class Point extends Data
{
    public function __construct(
        public int $x,
    ) {}
}

// Data declares $_additional, so a property of that name stops.
class Extra extends Data
{
    public function __construct(
        public int $_additional,
    ) {}
}

// Only Data itself as the direct parent converts.
class Deeper extends Point
{
}

// PHP rejects a list of parents; Mago's parser accepts it.
class Twice extends Data, Data
{
}

function parentCall(Point $p): int
{
    $p->toArray();
    return $p->x;
}

function viaFrom(): int
{
    return Point::from(1)->x;
}

function anyData(Data $d): int
{
    return 1;
}

function isData(Point $p): bool
{
    return $p instanceof Data;
}

// A method of Data's own name would override it, in any letter case.
class Own extends Data
{
    /** @return array<string, mixed> */
    public function toArray(): array
    {
        return [];
    }
}

// The parent named in another letter case is still Data.
class Shout extends \SPATIE\laraveldata\DATA
{
    public static function FROM(int $x): int
    {
        return $x;
    }
}

// PHP rejects a readonly class with a parent that is not readonly.
readonly class Frozen extends Data
{
}

class Context extends Data
{
    public function __construct(
        public int $_dataContext,
    ) {}

    public function extra(): int
    {
        return count($this->_additional);
    }
}

echo "x\n";

// With Composer's autoloader PHP declares it only when execution gets here.
class Late extends Data
{
}
