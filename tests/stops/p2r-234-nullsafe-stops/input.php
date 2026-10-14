<?php

declare(strict_types=1);

final readonly class Leaf
{
    public function __construct(public string $name) {}

    public function greet(string $who): string
    {
        return $this->name . $who;
    }

    public function hello(): void {}
}

final readonly class Branch
{
    public function __construct(public ?Leaf $leaf) {}
}

final class Counter
{
    public int $n = 0;

    public function bump(): int
    {
        $this->n += 1;
        return $this->n;
    }
}

function notNull(Leaf $leaf): string
{
    return $leaf?->name;
}

function chained(?Branch $branch): ?string
{
    return $branch?->leaf?->name;
}

function calledArgument(?Leaf $leaf, string $who): ?string
{
    return $leaf?->greet(strtoupper($who));
}

function changing(?Counter $c): ?int
{
    return $c?->bump();
}

function sameName(?Leaf $leaf): ?string
{
    return $leaf?->greet($leaf->name);
}

function voidCall(?Leaf $leaf): void
{
    $leaf?->hello();
}

function onCall(?Branch $branch): ?string
{
    return make()?->name;
}

function make(): ?Leaf
{
    return null;
}

final readonly class Risky
{
    public function __construct(public Leaf $leaf) {}

    public function check(): string
    {
        throw new RuntimeException('no');
    }
}

function throwing(?Risky $r): ?string
{
    return $r?->check();
}

function chainedPlain(?Risky $r): ?string
{
    return $r?->leaf->name;
}

function tested(?Leaf $leaf): bool
{
    return isset($leaf?->name) || empty($leaf?->name);
}

function joined(?Leaf $leaf): string
{
    return "name: {$leaf?->name}";
}
