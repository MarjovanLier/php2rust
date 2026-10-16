<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Collection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Collection as SupportCollection;

final class Code extends Model
{
    protected $fillable = ['name', 'kind'];

    public $timestamps = false;
}

// A collection as a property of a readonly class, through an interface method, as a
// parameter of Laravel's base collection, and as a parameter the function assigns to.
interface CodeSource
{
    /**
     * @return Collection<int, Code>
     */
    public function codes(): Collection;
}

final readonly class Batch implements CodeSource
{
    /**
     * @param Collection<int, Code> $items
     */
    public function __construct(public string $label, public Collection $items)
    {
    }

    /**
     * @return Collection<int, Code>
     */
    public function codes(): Collection
    {
        return $this->items;
    }
}

/**
 * @param SupportCollection<int, Code> $codes
 */
function joined(SupportCollection $codes): string
{
    $out = '';
    foreach ($codes as $code) {
        $out .= $code->name;
    }
    return $out;
}

/**
 * @param Collection<int, Code> $codes
 */
function counted(Collection $codes, bool $all): int
{
    if ($all) {
        $codes = Code::all();
    }
    return count($codes);
}

function described(Batch $source): string
{
    return joined($source->codes()) . ':' . $source->codes()->count();
}

Code::create(['name' => 'a', 'kind' => 'x']);
Code::create(['name' => 'b', 'kind' => 'y']);
Code::create(['name' => 'c', 'kind' => 'x']);
$batch = new Batch('xs', Code::query()->where('kind', 'x')->get());
$same = $batch->items;
echo $batch->label, ' ', joined($batch->items), ' ', joined($same), "\n";
echo counted($same, false), ' ', counted($same, true), ' ', described($batch), "\n";
