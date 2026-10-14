<?php

declare(strict_types=1);

/**
 * @implements ArrayAccess<string, string>
 */
final readonly class Parts implements ArrayAccess
{
    public function __construct(
        public string $first,
        public string $last,
    ) {}

    #[Override]
    public function offsetExists(mixed $offset): bool
    {
        return \in_array($offset, ['first', 'last'], true);
    }

    /**
     * @param mixed $offset The offset to read.
     */
    #[\Override]
    public function offsetGet(mixed $offset): string
    {
        return match ($offset) {
            'first' => $this->first,
            'last' => $this->last,
            default => throw new InvalidArgumentException(
                \sprintf('Invalid offset "%s".', \is_scalar($offset) ? (string) $offset : \gettype($offset)),
            ),
        };
    }

    #[Override]
    public function offsetSet(mixed $offset, mixed $value): void
    {
        throw new BadMethodCallException(
            \sprintf('Cannot set "%s" to "%s" (%s).', $offset, $value, \gettype($value)),
        );
    }

    #[Override()]
    public function offsetUnset(mixed $offset): void
    {
        throw new BadMethodCallException('Parts is immutable.');
    }
}

/**
 * @implements ArrayAccess<int, int>
 */
final class Counts implements ArrayAccess
{
    public int $total = 0;

    public function OffsetExists(mixed $offset): bool
    {
        return $offset >= 0 && $offset < 3;
    }

    public function offsetGet(mixed $offset): int
    {
        return $offset * 10 + $this->total;
    }

    public function offsetSet(mixed $offset, mixed $value): void
    {
        $this->total += $offset + $value;
    }

    public function offsetUnset(mixed $offset): void
    {
        $this->total -= $offset;
    }
}

function shout(string $s): string
{
    $u = (string) $s;
    $s .= '!';
    return $u . $s;
}

$p = new Parts('Ann', 'Lee');
echo $p->offsetGet('first'), ' ', $p->offsetGet('last'), "\n";
echo $p->offsetExists('first') ? 'yes' : 'no', ' ', $p->offsetExists('middle') ? 'yes' : 'no', "\n";
try {
    echo $p->offsetGet('middle'), "\n";
} catch (InvalidArgumentException $e) {
    echo $e->getMessage(), "\n";
}
try {
    $p->offsetSet('first', 'Bo');
} catch (BadMethodCallException $e) {
    echo $e->getMessage(), "\n";
}
try {
    $p->offsetUnset('last');
} catch (BadMethodCallException $e) {
    echo $e->getMessage(), "\n";
}
$c = new Counts();
$c->offsetSet(1, 5);
$c->offsetUnset(2);
echo $c->offsetGet(2), ' ', $c->offsetExists(4) ? 'in' : 'out', "\n";
$w = 'xyz';
$u = (string) $w;
echo $u, $w, "\n";
echo shout('hey'), "\n";
