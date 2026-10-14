<?php

declare(strict_types=1);

enum Mode
{
    case Fast;
    case Safe;

    const DEFAULT = self::Safe;
    public const int RETRIES = 3;
}

final readonly class Limits
{
    private const string TAG = 'limit';
    const float RATE = 2;
    const float ZERO = -0;
    final public const bool STRICT = true;
    const maxItems = -5;
    const Mode MODE = Mode::Fast;

    public static function tag(): string
    {
        return self::TAG . ':' . static::maxItems;
    }

    public function __construct(public int $n) {}

    public function rated(): float
    {
        return self::RATE * 1.5 + self::ZERO;
    }
}

echo Limits::tag(), "\n";
echo Mode::RETRIES + Limits::maxItems, "\n";
if (Limits::STRICT) {
    echo "strict\n";
}
if (Mode::DEFAULT === Mode::Safe) {
    echo "safe\n";
}
if (Limits::MODE === Mode::Fast) {
    echo "fast\n";
}
if (fdiv(1, (new Limits(1))->rated()) > 0.0) {
    echo "positive rate\n";
}
if (fdiv(1, Limits::ZERO) > 0.0) {
    echo "positive zero\n";
}
