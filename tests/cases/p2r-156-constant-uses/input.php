<?php

declare(strict_types=1);

namespace Acme;

use Acme\Mode as M;
use const PHP_INT_MAX as BIGGEST;
use function strlen as len, intdiv as halve;

enum Mode
{
    case Fast;
    case Safe;

    const self PREFERRED = self::Safe;

    public static function pick(int $n): self
    {
        return match ($n) {
            1 => static::Fast,
            default => self::PREFERRED,
        };
    }

    public function same(self $other): bool
    {
        return match ($this) {
            static::Fast => $other === Mode::Fast,
            self::Safe => $other === Mode::Safe,
        };
    }
}

final readonly class Text
{
    const TAG = 'tag';
    const int SIZE = 4;

    public function __construct(public string $label) {}

    public static function scaled(float $by): float
    {
        return $by * 2.5;
    }
}

$tag = Text::TAG;
$text = new Text(Text::TAG);
echo $tag, ' ', $text->label, ' ', len($tag), ' ', halve(9, 2), "\n";
echo intdiv(Text::SIZE * 10, 4), "\n";
if (M::pick(1)->same(Mode::Fast)) {
    echo "fast\n";
}
if (Text::scaled(Text::SIZE) > 9.5) {
    echo "scaled\n";
}
