<?php

declare(strict_types=1);

final readonly class Wallet
{
    public function __construct(
        public int $amount,
    ) {}

    public function take(int $n): int
    {
        if ($n > $this->amount) {
            throw new OverflowException("cannot take $n", 7);
        }
        return $this->amount - $n;
    }

    public static function make(int $n): self
    {
        if ($n < 0) {
            throw new DomainException('a wallet cannot hold less than nothing');
        }
        return new self($n);
    }
}

function half(int $n): int
{
    if ($n % 2 !== 0) {
        throw new InvalidArgumentException("odd: $n");
    }
    return intdiv($n, 2);
}

function quarter(int $n): int
{
    return half(half($n));
}

function check(int $n): void
{
    if ($n < 0) {
        throw new RangeException('negative', 3);
    }
    echo "ok $n\n";
}

function safe(int $n): int
{
    try {
        return half($n);
    } catch (DomainException $e) {
        return -1;
    }
}

function name(int $n): string
{
    return match ($n) {
        1 => 'one',
        2 => 'two',
        default => throw new OutOfRangeException("no name for $n"),
    };
}

function need(?string $s): string
{
    return $s ?? throw new UnexpectedValueException('missing');
}

function first(?int $x): int
{
    if ($x === null) {
        throw new LengthException('empty');
    }
    return $x + 1;
}

function relay(int $n): int
{
    try {
        return half($n);
    } catch (InvalidArgumentException $e) {
        echo 'relaying ', $e->getMessage(), "\n";
        throw $e;
    }
}

foreach ([8, 6, 3] as $n) {
    try {
        echo quarter($n), "\n";
    } catch (LogicException $e) {
        echo 'logic: ', $e->getMessage(), "\n";
    }
}
check(4);
try {
    check(-1);
} catch (RuntimeException | LogicException $e) {
    echo $e->getCode(), ' ', $e->getMessage(), "\n";
}
try {
    check(-5);
} catch (Throwable) {
    echo "caught\n";
}
try {
    echo safe(3), "\n";
} catch (InvalidArgumentException $e) {
    echo 'rethrown: ', $e->getMessage(), "\n";
}
echo name(2), "\n";
try {
    echo name(9), "\n";
} catch (Exception $e) {
    echo $e->getMessage(), "\n";
}
echo need('here'), "\n";
try {
    echo need(null), "\n";
} catch (UnexpectedValueException $e) {
    echo $e->getMessage(), "\n";
}
echo first(4), "\n";
try {
    echo first(null), "\n";
} catch (LengthException $e) {
    echo $e->getMessage(), "\n";
}
try {
    echo relay(5), "\n";
} catch (Exception $e) {
    echo 'relayed ', $e->getMessage(), "\n";
}
try {
    $r = half(7);
} catch (Exception $e) {
    $r = -1;
}
echo $r, "\n";
$w = Wallet::make(10);
echo $w->take(4), "\n";
try {
    echo $w->take(40), "\n";
} catch (OverflowException $e) {
    echo $e->getCode(), ': ', $e->getMessage(), "\n";
}
try {
    Wallet::make(-1);
} catch (DomainException $e) {
    echo $e->getMessage(), "\n";
}
