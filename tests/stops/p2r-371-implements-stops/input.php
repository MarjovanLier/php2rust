<?php

declare(strict_types=1);

namespace App;

interface Labelled
{
    public function label(): string;
}

// An interface PHP has built in stands beside the input's own.
final readonly class Pair implements Labelled, \Countable
{
    public function label(): string
    {
        return 'pair';
    }

    public function count(): int
    {
        return 2;
    }
}

// A wider parameter, a narrower return or an added parameter gives another Rust signature.
interface Greeter
{
    public function greet(string $name): ?string;
}

final readonly class Wide implements Greeter
{
    public function greet(?string $name): ?string
    {
        return $name;
    }
}

final readonly class Narrow implements Greeter
{
    public function greet(string $name): string
    {
        return $name;
    }
}

final readonly class Extra implements Greeter
{
    public function greet(string $name, int $times = 1): ?string
    {
        return $name;
    }
}

// A method that changes its object, or throws, has another Rust form.
interface Ticker
{
    public function tick(): int;
}

final class Counter implements Ticker
{
    public function __construct(public int $count)
    {
    }

    public function tick(): int
    {
        $this->count++;
        return $this->count;
    }
}

final readonly class Failing implements Ticker
{
    public function tick(): int
    {
        throw new \RuntimeException('no tick');
    }
}

// Two interfaces declare one method.
interface First
{
    public function value(): int;
}

interface Second
{
    public function value(): int;
}

final readonly class Both implements First, Second
{
    public function value(): int
    {
        return 1;
    }
}

// PHP matches method names ignoring letter case; the Rust names differ.
interface Titled
{
    public function getTitle(): string;
}

final readonly class Loud implements Titled
{
    public function GETTITLE(): string
    {
        return 'loud';
    }
}

// An interface named twice, and a method an enum cannot declare.
final readonly class Twice implements Labelled, labelled
{
    public function label(): string
    {
        return 'twice';
    }
}

interface Lookup
{
    public function tryFrom(int $value): int;
}

enum Status: int implements Lookup
{
    case Open = 1;
}

// An enum and a class implement one interface, whose methods take self in two ways.
interface Coded
{
    public function code(): int;
}

enum Level: int implements Coded
{
    case Low = 1;

    public function code(): int
    {
        return $this->value;
    }
}

final readonly class Plain implements Coded
{
    public function code(): int
    {
        return 0;
    }
}
