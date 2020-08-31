///
/// This file is an adapation of  Martin Sosic edlib.h to get a Rust idiomatic interface
/// simpler than the one generated by bingen, but ultimately call bingen generated interface.
/// All C structrures and functions are modified by appending "Rs" to their name.
/// We also avoid pointers.
///

use ::std::os::raw::c_char;
use ::std::slice;

use crate::bindings::*;

// Status codes
const EDLIB_STATUS_OK : u32 = 0;
const EDLIB_STATUS_ERROR : u32 = 1;

///
/// Alignment methods - how should Edlib treat gaps before and after query?
///
#[derive(Debug, Copy, Clone)]
pub enum EdlibAlignModeRs {
    ///
    /// Global method. This is the standard method.
    /// Useful when you want to find out how similar is first sequence to second sequence.
    ///
    EDLIB_MODE_NW,
    ///
    ///   Prefix method. Similar to global method, but with a small twist - gap at query end is not penalized.
    ///   What that means is that deleting elements from the end of second sequence is "free"!
    /// For example, if we had "AACT" and "AACTGGC", edit distance would be 0, because removing "GGC" from the end
    /// of second sequence is "free" and does not count into total edit distance. This method is appropriate
    /// when you want to find out how well first sequence fits at the beginning of second sequence.
    ///
    EDLIB_MODE_SHW,
    
    /// Infix method. Similar as prefix method, but with one more twist - gaps at query end and start are
    /// not penalized. What that means is that deleting elements from the start and end of second sequence is "free"!
    /// For example, if we had ACT and CGACTGAC, edit distance would be 0, because removing CG from the start
    /// and GAC from the end of second sequence is "free" and does not count into total edit distance.
    /// This method is appropriate when you want to find out how well first sequence fits at any part of
    /// second sequence.
    /// For example, if your second sequence was a long text and your first sequence was a sentence from that text,
    /// but slightly scrambled, you could use this method to discover how scrambled it is and where it fits in
    /// that text. In bioinformatics, this method is appropriate for aligning read to a sequence.
    ///
    EDLIB_MODE_HW
}

    ///
    /// Alignment tasks - what do you want Edlib to do?
    ///
#[derive(Debug, Copy, Clone)]
pub enum EdlibAlignTaskRs {
    /// Find edit distance and end locations
    EDLIB_TASK_DISTANCE,
    ///    Find edit distance, end locations and start locations.    
    EDLIB_TASK_LOC,
    /// Find edit distance, end locations and start locations and alignment path.       
    EDLIB_TASK_PATH
}

///
/// Describes cigar format.
/// see http://samtools.github.io/hts-specs/SAMv1.pdf
///see http://drive5.com/usearch/manual/cigar.html
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum EdlibCigarFormatRs {
    /// Match: 'M', Insertion: 'I', Deletion: 'D', Mismatch: 'M'.
    EDLIB_CIGAR_STANDARD,
    ///    Match: '=', Insertion: 'I', Deletion: 'D', Mismatch: 'X'.
    EDLIB_CIGAR_EXTENDED
}

/// Edit operations.
#[derive(Debug, Copy, Clone)]
pub enum EdlibEdopRs {
    /// Match
    EDLIB_EDOP_MATCH,
    /// Insertion to target = deletion from query
    EDLIB_EDOP_INSERT,
    /// Deletion from target = insertion to query.
    EDLIB_EDOP_DELETE,
     /// Mismatch.
    EDLIB_EDOP_MISMATCH
}


// #define EDLIB_EDOP_MATCH  0   //!< Match.
// #define EDLIB_EDOP_INSERT 1   //!< Insertion to target = deletion from query.
// #define EDLIB_EDOP_DELETE 2   //!< Deletion from target = insertion to query.
// #define EDLIB_EDOP_MISMATCH 3 //!< Mismatch.

// We use c_char here to be able to cast C pointer directly
/// Defines two given characters as equal.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EdlibEqualityPairRs {
    first : ::std::os::raw::c_char,
    second : ::std::os::raw::c_char,
}



//=================================================================================================
/// 
/// Configuration object for edlibAlign() function.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EdlibAlignConfigRs<'a> {
    /// Set k to non-negative value to tell edlib that edit distance is not larger than k.
    /// Smaller k can significantly improve speed of computation.
    /// If edit distance is larger than k, edlib will set edit distance to -1.
    /// Set k to negative value and edlib will internally auto-adjust k until score is found.
    k : i32,

    /// Alignment method.
    /// EDLIB_MODE_NW: global (Needleman-Wunsch)
    /// EDLIB_MODE_SHW: prefix. Gap after query is not penalized.
    /// EDLIB_MODE_HW: infix. Gaps before and after query are not penalized.
    ///
    mode : EdlibAlignModeRs,

    /// Alignment task - tells Edlib what to calculate. Less to calculate, faster it is.
    /// EDLIB_TASK_DISTANCE - find edit distance and end locations of optimal alignment paths in target.
    /// EDLIB_TASK_LOC - find edit distance and start and end locations of optimal alignment paths in target.
    /// EDLIB_TASK_PATH - find edit distance, alignment path (and start and end locations of it in target).
    ///
    task : EdlibAlignTaskRs,

    /// List of pairs of characters, where each pair defines two characters as equal.
    /// This way you can extend edlib's definition of equality (which is that each character is equal only
    /// to itself).
    /// This can be useful if you have some wildcard characters that should match multiple other characters,
    /// or e.g. if you want edlib to be case insensitive.
    /// Can be set to NULL if there are none.
    additionalequalities : &'a[EdlibEqualityPairRs],
}


impl <'a> EdlibAlignConfigRs<'a> {

    /// Helper method for easy construction of configuration object.
    /// return Configuration object filled with given parameters.
    pub fn new(k : i32, mode : EdlibAlignModeRs, 
                                task : EdlibAlignTaskRs,
                                additionalequalities :  &'a[EdlibEqualityPairRs]) -> Self {
        
        EdlibAlignConfigRs{k, mode, task, additionalequalities}
    }
}



impl <'a> Default for EdlibAlignConfigRs<'a> {
    ///      k = -1, mode = EDLIB_MODE_NW, task = EDLIB_TASK_DISTANCE, no additional equalities.
    fn default() -> Self {
        EdlibAlignConfigRs{k:-1, 
                            mode : EdlibAlignModeRs::EDLIB_MODE_NW,
                            task : EdlibAlignTaskRs::EDLIB_TASK_DISTANCE, 
                            additionalequalities : &[]}
    }
}


//================================================================================================

/// Container for results of alignment done by edlibAlign() function.
#[derive(Debug,Clone)]
pub struct EdlibAlignResultRs {
    /// EDLIB_STATUS_OK or EDLIB_STATUS_ERROR. If error, all other fields will have undefined values.
    status : u32,

    /// -1 if k is non-negative and edit distance is larger than k.
    editDistance : i32,

    /// Array of zero-based positions in target where optimal alignment paths end.
    /// If gap after query is penalized, gap counts as part of query (NW), otherwise not.
    /// Set to NULL if edit distance is larger than k.
    endLocations : Option<Vec<i32>>,

    /// Array of zero-based positions in target where optimal alignment paths start,
    /// they correspond to endLocations.
    /// If gap before query is penalized, gap counts as part of query (NW), otherwise not.
    /// Set to NULL if not calculated or if edit distance is larger than k.
    startLocations : Option<Vec<i32>>,

    
    /// Number of end (and start) locations.
    numLocations : usize,

    /// Alignment is found for first pair of start and end locations.
    /// Set to NULL if not calculated.
    /// Alignment is sequence of numbers: 0, 1, 2, 3.
    /// 0 stands for match.
    /// 1 stands for insertion to target.
    /// 2 stands for insertion to query.
    /// 3 stands for mismatch.
    /// Alignment aligns query to target from begining of query till end of query.
    /// If gaps are not penalized, they are not in alignment.
    alignment : Option<Vec<char>>,

    /// Length of alignment.
    alignmentLength : u32,

     /// Number of different characters in query and target together.
    alphabetLength : u32
}  // end of struct EdlibAlignResultRs



impl Default for  EdlibAlignResultRs {
    ///      k = -1, mode = EDLIB_MODE_NW, task = EDLIB_TASK_DISTANCE, no additional equalities.
    fn default() -> Self {
        EdlibAlignResultRs{ status : EDLIB_STATUS_OK, 
                            editDistance : 0,
                            endLocations : None,
                            startLocations : None,
                            numLocations : 0,
                            alignment : None,
                            alignmentLength : 0,
                            alphabetLength : 0,
        }
    }
}  // end impl Default for EdlibAlignResultRs




    
    /// Aligns two sequences (query and target) using edit distance (levenshtein distance).
    /// Through config parameter, this function supports different alignment methods (global, prefix, infix),
    /// as well as different modes of search (tasks).
    /// It always returns edit distance and end locations of optimal alignment in target.
    /// It optionally returns start locations of optimal alignment in target and alignment path,
    /// if you choose appropriate tasks.
    /// Parameters:
    ///     - query  : First sequence.
    ///     - target : Second sequence.
    ///     - config : Additional alignment parameters, like alignment method and wanted results.
    ///  Result of alignment, which can contain edit distance, start and end locations and alignment path.
    /// Note:
    ///  Rust interface causes clone of start/end locations and ensures i32 representation and so transfer memory responsability to Rust.
    
    pub fn edlibAlignRs(query : &[u8], target : &[u8], config_rs : &EdlibAlignConfigRs) -> EdlibAlignResultRs {
        // real work here
        // get pointers to query and target to EdlibEqualityPair form config
        let mut config_c = unsafe { edlibDefaultAlignConfig() };
        config_c.k = config_rs.k as  ::std::os::raw::c_int;
        config_c.mode = match config_rs.mode {
            EdlibAlignModeRs::EDLIB_MODE_NW => 0,
            EdlibAlignModeRs::EDLIB_MODE_SHW => 1,
            EdlibAlignModeRs::EDLIB_MODE_HW => 2,
        };
        config_c.additionalEqualitiesLength = config_rs.additionalequalities.len() as ::std::os::raw::c_int;
        if config_c.additionalEqualitiesLength > 0 {
            config_c.additionalEqualities = config_rs.additionalequalities.as_ptr() as *const EdlibEqualityPair;
        }
        else {
            config_c.additionalEqualities = ::std::ptr::null::<EdlibEqualityPair>();
        }
        // Recast to EdlibAlignResultRs
        let res_c : EdlibAlignResult = unsafe { edlibAlign(query.as_ptr() as *const ::std::os::raw::c_char,
                                            query.len() as ::std::os::raw::c_int, 
                                            target.as_ptr() as *const ::std::os::raw::c_char, 
                                            target.len() as ::std::os::raw::c_int,
                                            // now config
                                            config_c
                                        )} ;
        // go back to EdlibAlignResultRs. Clone incurs some cost. Should go to impl From<EdlibAlignResult>
        let mut align_res_rs = EdlibAlignResultRs::default();
        align_res_rs.status = res_c.status as u32;
        align_res_rs.editDistance = res_c.editDistance as i32;
        align_res_rs.numLocations = res_c.numLocations as usize;
        // get  ::std::os::raw::c_int slices for endLocations
        if align_res_rs.numLocations > 0 {
            let s_end = unsafe { slice::from_raw_parts(res_c.endLocations, align_res_rs.numLocations) };
            align_res_rs.endLocations = Some(s_end.into_iter().map(|l| *l as i32).collect());
            let s_start = unsafe { slice::from_raw_parts(res_c.startLocations, align_res_rs.numLocations) };
            align_res_rs.startLocations = Some(s_start.into_iter().map(|l| *l as i32).collect());
        }
        // Free C datas
        unsafe { edlibFreeAlignResult(res_c); };
        //
        align_res_rs
    }



    
    /// Builds cigar string from given alignment sequence.
    ///  @param [in] alignment  Alignment sequence.
    //  *     0 stands for match.
    //  *     1 stands for insertion to target.
    //  *     2 stands for insertion to query.
    //  *     3 stands for mismatch.
    //  * @param [in] alignmentLength
    //  * @param [in] cigarFormat  Cigar will be returned in specified format.
    ///
    //  * @return Cigar string.
    ///
    ///     I stands for insertion.
    ///     D stands for deletion.
    ///     X stands for mismatch. (used only in extended format)
    ///    = stands for match. (used only in extended format)
    ///     M stands for (mis)match. (used only in standard format)
    //  *     String is null terminated.
    //  *     Needed memory is allocated and given pointer is set to it.
    //  *     Do not forget to free it later using free()!
    // 
    pub fn edlibAlignmentToCigarRs(alignment : &[u8], cigarFormat : &EdlibCigarFormat) {
        println!("not yet iplmeented");

    }




