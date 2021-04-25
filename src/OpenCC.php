<?php
namespace FastFFI\Opencc;

use FFI;
use RuntimeException;

class OpenCC
{
    /**
     * @var ?OpenCC
     */
    private static ?OpenCC $cc = null;

    /**
     * @var FFI
     */
    protected FFI $ffi;

    /**
     * @var string
     */
    protected string $openccDataDir = __DIR__ . '/../opencc';

    /**
     * @var FFI\CData
     */
    private $opencc;

    /**
     * OpenCC constructor.
     * @param string|null $openccDataDir
     */
    private function __construct(string $openccDataDir = null)
    {
        if (ini_get('ffi.enable') == false) {
            throw new RuntimeException("请设置php.ini中的ffi.enable参数");
        }
        $this->ffi = $this->makeFFI();
        if ($openccDataDir && file_exists($openccDataDir)) {
            $this->openccDataDir = $openccDataDir;
        }
        $this->opencc = $this->ffi->with_dict_path($this->openccDataDir);
    }

    /**
     *
     */
    public function __destruct()
    {
        // TODO: Implement __destruct() method.
        $this->ffi->free_opencc($this->opencc);
    }

    /**
     * 简体转繁体
     *
     * @param string $str
     * @return string
     */
    public function s2t(string $str): string
    {
        if (empty($str)) {
            return "";
        }
        $char = $this->ffi->s2t($str, $this->opencc);
        return $this->convert($char);
    }

    /**
     * 繁体转简体
     *
     * @param string $str
     * @return string
     */
    public function t2s(string $str): string
    {
        if (empty($str)) {
            return "";
        }
        $char = $this->ffi->t2s($str, $this->opencc);
        return $this->convert($char);
    }

    /**
     * 简体转台繁正体
     *
     * @param string $str
     * @return string
     */
    public function s2tw(string $str): string
    {
        if (empty($str)) {
            return "";
        }
        $char = $this->ffi->s2tw($str, $this->opencc);
        return $this->convert($char);
    }

    /**
     * 台繁正体转简体
     *
     * @param string $str
     * @return string
     */
    public function tw2s(string $str): string
    {
        if (empty($str)) {
            return "";
        }
        $char = $this->ffi->tw2s($str, $this->opencc);
        return $this->convert($char);
    }

    /**
     * 简单转香港繁体
     *
     * @param string $str
     * @return string
     */
    public function s2hk(string $str): string
    {
        if (empty($str)) {
            return "";
        }
        $char = $this->ffi->s2hk($str, $this->opencc);
        return $this->convert($char);
    }

    /**
     * 香港繁体转简体
     *
     * @param string $str
     * @return string
     */
    public function hk2s(string $str): string
    {
        if (empty($str)) {
            return "";
        }
        $char = $this->ffi->hk2s($str, $this->opencc);
        return $this->convert($char);
    }


    /**
     * @param string|null $dictPath
     * @return static
     */
    public static function new(string $dictPath = null): OpenCC
    {
        if (self::$cc == null) {
            self::$cc = new static($dictPath);
        }
        return self::$cc;
    }

    /**
     *
     */
    private function __clone()
    {

    }



    /**
     * @param FFI\CData $CData
     * @return string
     */
    private function convert(FFI\CData $CData)
    {
        $result = FFI::string($CData);
        $this->ffi->free_pointer($CData);
        return $result;
    }

    /**
     * @return FFI
     */
    private function makeFFI(): FFI
    {
        return FFI::cdef(
            file_get_contents(__DIR__ . '/../lib/ffi_opencc.h'),
            $this->defaultLibraryPath()
        );
    }

    /**
     * @return string
     */
    private function defaultLibraryPath(): string
    {
        if (PHP_INT_SIZE !== 8) {
            throw new RuntimeException('不支持32位系统，请自行编译lib文件');
        }
        $suffix = PHP_SHLIB_SUFFIX;
        if (PHP_OS == 'Darwin') {
            $suffix = 'dylib';
        }
        $filepath = __DIR__ . '/../lib/libffi_opencc.' . $suffix;
        if (file_exists($filepath)) {
            return realpath($filepath);
        }
        throw new RuntimeException('不支持的系统，请自行编译lib文件');
    }
}
