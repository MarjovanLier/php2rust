<?php

declare(strict_types=1);

namespace Blog;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\ModelNotFoundException;

// A small blog and contact site, written by hand for the harness: posts with unique slugs
// and drafts, tags found or created by slug,
// pages by slug, contact submissions with a column default and a read flag, and the SEO
// data of a page.
final class Post extends Model
{
    protected $fillable = ['title', 'slug', 'excerpt', 'published_at'];

    public $timestamps = false;
}

final class Tag extends Model
{
    protected $fillable = ['name', 'slug'];

    public $timestamps = false;
}

final class Page extends Model
{
    protected $fillable = ['slug', 'title', 'content'];

    public $timestamps = false;
}

final class ContactSubmission extends Model
{
    protected $fillable = ['name', 'email', 'phone', 'message', 'preferred_contact', 'read_at'];

    public $timestamps = false;
}

final readonly class SeoData
{
    public function __construct(
        public string $title,
        public string $description,
        public string $type = 'website',
        public ?string $image = null,
        public bool $noIndex = false,
    ) {}

    public function fullTitle(): string
    {
        if (str_contains($this->title, 'Oakwood')) {
            return $this->title;
        }
        return $this->title . ' - Oakwood Studio';
    }

    public function imageUrl(): string
    {
        $image = $this->image;
        if ($image === null) {
            return '/images/og-default.jpg';
        }
        if (str_starts_with($image, 'http')) {
            return $image;
        }
        return '/storage/' . $image;
    }
}

function slugFor(string $title): string
{
    return str_replace(' ', '-', strtolower(trim($title, ' ')));
}

// The first free slug: the title's own, then with -1, -2 and on, as a counter does.
function uniqueSlug(string $title): string
{
    $base = slugFor($title);
    $slug = $base;
    $counter = 1;
    while (Post::query()->where('slug', $slug)->first() !== null) {
        $slug = $base . '-' . $counter;
        $counter++;
    }
    return $slug;
}

function write(string $title, string $excerpt, ?string $publishedAt): string
{
    $post = Post::create([
        'title' => $title,
        'slug' => uniqueSlug($title),
        'excerpt' => $excerpt,
        'published_at' => $publishedAt,
    ]);
    return $post->slug;
}

function isDraft(string $slug): bool
{
    $post = Post::query()->where('slug', $slug)->firstOrFail();
    return $post->published_at === null;
}

function show(string $slug): string
{
    try {
        $post = Post::query()->where('slug', $slug)->firstOrFail();
        return $post->title . ': ' . $post->excerpt;
    } catch (ModelNotFoundException $e) {
        return '404 ' . $e->getMessage();
    }
}

/**
 * @return list<string>
 */
function publishedTitles(): array
{
    $found = [];
    foreach (Post::all() as $post) {
        if ($post->published_at !== null) {
            $found[] = $post->title;
        }
    }
    return $found;
}

function tagFor(string $name): int
{
    $slug = slugFor($name);
    $tag = Tag::query()->where('slug', $slug)->first();
    if ($tag !== null) {
        return $tag->id;
    }
    return Tag::create(['name' => $name, 'slug' => $slug])->id;
}

function addPage(string $slug, string $title, string $content): int
{
    return Page::create(['slug' => $slug, 'title' => $title, 'content' => $content])->id;
}

function pageTitle(string $slug): string
{
    $page = Page::query()->where('slug', $slug)->first();
    if ($page === null) {
        return 'no page';
    }
    return $page->title;
}

function submit(string $name, string $email, ?string $phone, string $message, string $preferred): int
{
    $submission = ContactSubmission::create([
        'name' => $name,
        'email' => $email,
        'phone' => $phone,
        'message' => $message,
        'preferred_contact' => $preferred,
    ]);
    return $submission->id;
}

function preferredOf(int $id): string
{
    return ContactSubmission::findOrFail($id)->preferred_contact;
}

function phoneOf(int $id): ?string
{
    return ContactSubmission::findOrFail($id)->phone;
}

function unreadCount(): int
{
    return ContactSubmission::query()->where('read_at', null)->get()->count();
}

function seoTitle(string $title): string
{
    $seo = new SeoData(title: $title, description: 'A page.');
    return $seo->fullTitle();
}

function seoImage(?string $image): string
{
    $seo = new SeoData(title: 'Post', description: 'A post.', type: 'article', image: $image);
    return $seo->imageUrl();
}

function seoType(bool $article): string
{
    $seo = $article
        ? new SeoData(title: 'Post', description: 'A post.', type: 'article')
        : new SeoData(title: 'Home', description: 'Welcome.');
    return $seo->type;
}
