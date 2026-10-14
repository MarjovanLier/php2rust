<?php

declare(strict_types=1);

enum Gender: string
{
    case Female = 'female';
    case Male = 'male';
}

/**
 * @implements ArrayAccess<string, string>
 */
final readonly class DateComponents implements ArrayAccess
{
    public function __construct(
        public string $year,
        public string $month,
        public string $day,
    ) {}

    /**
     * @return array{year: string, month: string, day: string}
     */
    public function toArray(): array
    {
        return ['year' => $this->year, 'month' => $this->month, 'day' => $this->day];
    }

    public function offsetExists(mixed $offset): bool
    {
        return \in_array($offset, ['year', 'month', 'day'], true);
    }

    public function offsetGet(mixed $offset): string
    {
        return match ($offset) {
            'year' => $this->year,
            'month' => $this->month,
            'day' => $this->day,
            default => throw new InvalidArgumentException('Invalid offset.'),
        };
    }

    public function offsetSet(mixed $offset, mixed $value): void
    {
        throw new BadMethodCallException('immutable');
    }

    public function offsetUnset(mixed $offset): void
    {
        throw new BadMethodCallException('immutable');
    }
}

/**
 * @implements ArrayAccess<string, bool|array{year: string, month: string, day: string}|string|null>
 */
final readonly class IdResult implements ArrayAccess
{
    public function __construct(
        public bool $valid,
        public ?DateComponents $dateComponents,
        public ?Gender $gender,
    ) {}

    /**
     * @param mixed $offset The offset to check.
     */
    #[Override]
    public function offsetExists(mixed $offset): bool
    {
        return \in_array($offset, ['valid', 'date_components', 'gender'], true);
    }

    /**
     * @param mixed $offset The offset to retrieve.
     *
     * @return bool|array{year: string, month: string, day: string}|string|null
     */
    #[Override]
    public function offsetGet(mixed $offset): bool|array|string|null
    {
        return match ($offset) {
            'valid' => $this->valid,
            'date_components' => $this->dateComponents?->toArray(),
            'gender' => $this->gender?->value,
            default => throw new InvalidArgumentException(
                \sprintf('Invalid offset "%s".', \is_scalar($offset) ? (string) $offset : \gettype($offset)),
            ),
        };
    }

    /**
     * @param mixed $offset The offset.
     * @param mixed $value  The value.
     */
    #[Override]
    public function offsetSet(mixed $offset, mixed $value): void
    {
        $offsetStr = \is_scalar($offset) ? (string) $offset : \gettype($offset);
        $valueStr = \is_scalar($value) ? (string) $value : \gettype($value);

        throw new BadMethodCallException(
            \sprintf('Cannot set "%s" to "%s": IdResult is immutable.', $offsetStr, $valueStr),
        );
    }

    /**
     * @param mixed $offset The offset.
     */
    #[Override]
    public function offsetUnset(mixed $offset): void
    {
        throw new BadMethodCallException('immutable');
    }
}

/**
 * @param bool|array{year: string, month: string, day: string}|string|null $v
 */
function show(bool|array|string|null $v): string
{
    return \is_scalar($v) ? (string) $v : \gettype($v);
}

function text(int|string|null $v): string
{
    return (string) $v;
}

function scalar(int|string|bool $v): bool
{
    return is_scalar($v);
}

$r = new IdResult(true, new DateComponents('1990', '01', '15'), Gender::Male);
echo show($r->offsetGet('valid')), ' ', show($r->offsetGet('gender')), ' ', show($r->offsetGet('date_components')), "\n";
$n = new IdResult(false, null, null);
echo show($n->offsetGet('valid')), '|', show($n->offsetGet('gender')), '|', show($n->offsetGet('date_components')), "\n";
try {
    $r->offsetSet('gender', 'x');
} catch (BadMethodCallException $e) {
    echo $e->getMessage(), "\n";
}
try {
    $r->offsetSet('valid', true);
} catch (BadMethodCallException $e) {
    echo $e->getMessage(), "\n";
}
try {
    $r->offsetSet('valid', null);
} catch (BadMethodCallException $e) {
    echo $e->getMessage(), "\n";
}
try {
    echo show($r->offsetGet('nope')), "\n";
} catch (InvalidArgumentException $e) {
    echo $e->getMessage(), "\n";
}
echo text(-7), text('x'), text(null), ' ', scalar(false) ? 'yes' : 'no', "\n";
