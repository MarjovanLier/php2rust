<?php

declare(strict_types=1);

// Only an enum implements it, so its trait takes self as an enum method does.
interface Labelable
{
    public function label(): string;
}

// Classes implement these, so their traits take &self.
interface Measured
{
    function size(int $extra): int;
    public function scaled(float $factor): float;
    public function fits(Crate $other): bool;
}

interface Named
{
    public function name(string $prefix): string;
    /** @param list<int> $weights */
    public function total(array $weights): int;
}

// No class implements it, and its trait still converts.
interface Printable
{
    public function printed(): string;
}

enum Kind: int implements Labelable
{
    case Quote = 0;
    case CreditMemo = 3;

    public function label(): string
    {
        return match ($this) {
            self::Quote => 'Quote',
            self::CreditMemo => 'Credit Memo',
        };
    }

    public function code(): int
    {
        return $this->value;
    }
}

final readonly class Crate implements Measured, Named
{
    public function __construct(public int $width, public string $title)
    {
    }

    public function size(int $extra): int
    {
        return $this->width + $extra;
    }

    public function scaled(float $factor): float
    {
        return $factor * 2.0;
    }

    public function fits(Crate $other): bool
    {
        return $this->width <= $other->width;
    }

    public function name(string $prefix): string
    {
        return $prefix . $this->title;
    }

    // Mago gives it the list type of the interface's docblock.
    public function total(array $weights): int
    {
        $sum = $this->width;
        foreach ($weights as $weight) {
            $sum += $weight;
        }
        return $sum;
    }

    public function area(): int
    {
        return $this->width * $this->width;
    }
}

function describe(Kind $kind, Crate $item): string
{
    return $kind->label() . ' ' . $item->name('#') . ' ' . $item->size(2);
}

$small = new Crate(3, 'crate');
$large = new Crate(5, 'box');
echo describe(Kind::CreditMemo, $small), "\n";
echo Kind::Quote->label(), ' ', Kind::Quote->code(), ' ', $small->area(), "\n";
echo $small->scaled(1.5) > 2.5 ? 'big' : 'small', ' ', $small->fits($large) ? 'fits' : 'too wide', ' ', $large->total([1, 2]), "\n";
