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
    protected $fillable = ['name', 'rank'];

    public $timestamps = false;
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
