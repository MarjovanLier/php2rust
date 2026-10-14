<?php

declare(strict_types=1);

enum Colour: string
{
    case CornflowerBlue = 'cb';
    case MediumSeaGreen = 'msg';
    case LightGoldenrodYellow = 'lgy';
    case PapayaWhip = 'pw';
    case Black = 'k';

    const PRIMARY = self::Black;

    public function isPastel(): bool
    {
        return match ($this) {
            self::CornflowerBlue, self::MediumSeaGreen, self::LightGoldenrodYellow, self::PapayaWhip => true,
            default => false,
        };
    }

    public static function fromCode(string $code): self
    {
        return self::from($code);
    }

    public static function countPastels(self $colour, int $rounds): int
    {
        $total = 0;
        for ($i = 0; $i < $rounds; $i++) {
            $pastel = match ($colour) {
                self::CornflowerBlue, self::MediumSeaGreen, self::LightGoldenrodYellow, self::PapayaWhip => 1,
                self::Black => 0,
            };
            $total += $pastel;
        }
        return $total;
    }
}

enum Unit
{
    case Metre;

    const BASE = self::Metre;
}

final readonly class Swatch
{
    const int SIZE = 4;

    public function __construct(public Colour $colour) {}

    public function name(): string
    {
        return $this->colour->value;
    }
}

if (Colour::fromCode('pw')->isPastel()) {
    echo "pastel\n";
}
if (!Colour::PRIMARY->isPastel()) {
    echo "bold\n";
}
if (Unit::BASE === Unit::Metre) {
    echo "metre\n";
}
echo Colour::countPastels(Colour::PapayaWhip, 3), "\n";
echo (new Swatch(Colour::MediumSeaGreen))->name(), Swatch::SIZE, "\n";
