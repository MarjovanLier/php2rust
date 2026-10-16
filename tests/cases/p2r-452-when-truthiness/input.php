<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection;
use Illuminate\Database\Eloquent\Model;

enum Kind: string
{
    case Main = 'main';
    case Side = 'side';
}

final class Code extends Model
{
    protected $fillable = ['name', 'rank', 'note', 'level'];

    public $timestamps = false;
}

final class Filter
{
    public function __construct(public ?string $note, public ?int $level)
    {
    }
}

// PHP's truthiness of each value type decides whether a when() callback filters: a bool,
// a list, an enum case and a plain int (P2R-452).
/**
 * @param list<string> $tags
 * @return Collection<int, Code>
 */
function picked(bool $flag, array $tags, ?Kind $kind, int $rank): Collection
{
    return Code::query()
        ->when($flag, fn (Builder $query) => $query->where('rank', 1))
        ->when($tags, fn (Builder $query) => $query->where('name', 'a'))
        ->when($kind, fn (Builder $query) => $query->where('rank', 1))
        ->when($rank, fn (Builder $query, int $rank) => $query->where('rank', $rank))
        ->get();
}

// Option values of a bool and a list, a closure of two statements in brackets, null in a
// callback where, property reads as values, and nullable fields compared with the bound
// value (P2R-451, P2R-452).
/**
 * @param list<string>|null $tags
 * @return Collection<int, Code>
 */
function optional(?bool $flag, ?array $tags, Filter $filter): Collection
{
    return Code::query()
        ->when($flag, (function (Builder $query): void {
            $query->where('rank', 1);
            $query->where('level', null);
        }))
        ->when($tags, fn (Builder $query) => $query->where('note', null))
        ->when($filter->note, fn (Builder $query, string $note) => $query->where('note', $note))
        ->when($filter->level, fn (Builder $query, int $level) => $query->where('level', $level))
        ->get();
}

/**
 * @param Collection<int, Code> $codes
 */
function names(Collection $codes): string
{
    $out = '';
    foreach ($codes as $code) {
        $out .= $code->name;
    }
    return $out === '' ? '-' : $out;
}

Code::create(['name' => 'a', 'rank' => 1]);
Code::create(['name' => 'b', 'rank' => 2]);
Code::create(['name' => 'a', 'rank' => 2]);
echo names(picked(false, [], null, 0)), ' ', names(picked(true, [], null, 0)), "\n";
echo names(picked(false, ['x'], null, 0)), ' ', names(picked(false, [''], null, 0)), "\n";
echo names(picked(false, [], Kind::Side, 0)), ' ', names(picked(false, [], null, 2)), ' ', names(picked(true, ['x'], Kind::Main, 1)), "\n";
Code::create(['name' => 'c', 'rank' => 1, 'note' => 'n', 'level' => 3]);
Code::create(['name' => 'd', 'rank' => 2, 'note' => 'n']);
$none = new Filter(null, null);
echo names(optional(null, null, $none)), ' ', names(optional(true, null, $none)), ' ', names(optional(false, ['x'], $none)), "\n";
echo names(optional(null, null, new Filter('n', null))), ' ', names(optional(null, null, new Filter('n', 3))), ' ', names(optional(null, [], new Filter('0', 0))), "\n";
